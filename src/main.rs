use std::collections::{HashMap, HashSet};
use std::fs;
use std::sync::Arc;

use axum::http::StatusCode;
use axum::response::{IntoResponse, Redirect, Response};
use axum::{
    extract::{Query, State},
    routing::get,
    Router, Server,
};
use axum_sessions::SameSite;
use axum_sessions::{async_session::MemoryStore, extractors::WritableSession, SessionLayer};
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use diesel::{PgConnection, SqliteConnection};
use directories::ProjectDirs;
use eyre::Result;
use futures::future;
use hyper::Method;
use hyper::{body::Buf, Body, Client, Request};
use hyper_tls::HttpsConnector;
use rand::{thread_rng, RngCore};
use rfesi::prelude::*;
use serde::Deserialize;
use serde::Serialize;
use tokio::sync::Mutex;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

use crate::config::Config;
use crate::models::eve::*;
use crate::models::state::*;

mod config;
pub mod models;
pub mod schema;

const VERIFY_URL: &str = "https://login.eveonline.com/oauth/verify";
const USER_AGENT: &str = "economeve/1.0";

#[derive(Debug, Serialize)]
struct Material {
    name: String,
    quantity: i32,
}

#[derive(Deserialize, Serialize)]
pub struct EveRedirectRequest {
    pub code: String,
    pub state: String,
}

#[derive(Deserialize, Serialize)]
pub struct QueryRequest {
    pub material: String,
    pub quantity: u32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(dead_code)]
struct VerifiedCharacter {
    #[serde(rename = "CharacterID")]
    pub character_id: i32,
    pub character_name: String,
    pub expires_on: String,
    pub scopes: String,
    pub token_type: String,
    pub character_owner_hash: String,
    pub intellectual_property: String,
}

struct AppState {
    config: Config,
    eve_db: Mutex<PgConnection>,
    state_db: Mutex<SqliteConnection>,
}

fn blueprint_for(inv_type: &InvType, conn: &mut PgConnection) -> Option<IndustryActivityProduct> {
    IndustryActivityProduct::belonging_to(&inv_type)
        .select(IndustryActivityProduct::as_select())
        .get_result(conn)
        .ok()
}

fn materials_for(
    blueprint: &IndustryActivityProduct,
    quantity: u32,
    conn: &mut PgConnection,
) -> Result<Vec<(InvType, u32)>> {
    use self::schema::eve::industry_activity_materials;
    use self::schema::eve::inv_types;

    let quantity = (quantity as f32 / blueprint.quantity as f32).ceil() as u32;

    Ok(industry_activity_materials::table
        .filter(industry_activity_materials::type_id.eq(blueprint.type_id))
        .select(IndustryActivityMaterial::as_select())
        .load(conn)?
        .into_iter()
        .map(|material| {
            inv_types::table
                .filter(inv_types::type_id.eq(material.material_type_id))
                .select(InvType::as_select())
                .get_result(conn)
                .map(|r| (r, quantity * material.quantity as u32))
        })
        .collect::<std::result::Result<Vec<_>, diesel::result::Error>>()?)
}

async fn verify_character(token: String) -> Result<VerifiedCharacter> {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    let request = Request::builder()
        .method(Method::GET)
        .uri(VERIFY_URL)
        .header("User-Agent", USER_AGENT)
        .header("Authorization", format!("Bearer {}", token))
        .body(Body::empty())?;
    let response = client.request(request).await?;
    let body = hyper::body::aggregate(response).await?;

    Ok(serde_json::from_reader(body.reader())?)
}

async fn character_assets(id: i32, esi: &Esi) -> Result<Vec<u64>> {
    let assets = esi.group_assets().get_character_assets(id as u64).await?;

    Ok(assets.iter().map(|asset| asset.type_id).collect())
}

fn state_db() -> Result<SqliteConnection> {
    let dirs = ProjectDirs::from("dev", "outergod", "economeve")
        .expect("Could not determine home directory! Quitting");
    let state_dir = dirs.data_dir();
    fs::create_dir_all(state_dir)?;

    Ok(SqliteConnection::establish(&format!(
        "sqlite://{}/{}",
        state_dir
            .to_str()
            .expect("Invalid characters in path! Quitting"),
        "state.sqlite"
    ))?)
}

async fn character_esi(character: Character, mut esi: Esi) -> Result<(i32, Esi)> {
    println!("{} vs {}", character.expiry.and_utc(), Utc::now());
    if Utc::now() > character.expiry.and_utc() {
        esi.use_refresh_token(&character.refresh_token).await?;
    } else {
        esi.access_token = Some(character.access_token.clone());
        esi.access_expiration = Some(character.expiry.timestamp_millis() as u128);
        esi.refresh_token = Some(character.refresh_token.clone());
    }

    Ok((character.id, esi))
}

async fn query(
    Query(query): Query<QueryRequest>,
    State(state): State<Arc<AppState>>,
) -> Result<String, Response> {
    use crate::schema::eve::inv_types;
    use crate::schema::state::characters;

    let mut state_db = state.state_db.lock().await;
    let mut eve_db = state.eve_db.lock().await;
    let config = &state.config;

    let mut esi = esi_client(config)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response())?;

    esi.update_spec()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response())?;

    let esis: Vec<_> = future::try_join_all(
        characters::table
            .select(Character::as_select())
            .load(&mut *state_db)
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response())?
            .into_iter()
            .map(|c| character_esi(c, esi.clone())),
    )
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response())?;

    let mut assets = HashSet::new();
    for (character_id, esi) in esis.into_iter() {
        println!("Assets for character {}", character_id);
        let mut a = character_assets(character_id, &esi)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response())?
            .into_iter();
        assets.extend(&mut a);
    }

    println!("{:?}", assets);

    let root = inv_types::table
        .filter(inv_types::type_name.eq(Some(query.material)))
        .select(InvType::as_select())
        .get_result(&mut *eve_db)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response())?;

    let mut stack = vec![(root, query.quantity)];
    let mut result = HashMap::new();

    while let Some((current, quantity)) = stack.pop() {
        let name = current.type_name.clone().unwrap();

        match blueprint_for(&current, &mut eve_db) {
            Some(blueprint) if assets.contains(&(blueprint.type_id as u64)) => {
                println!("Found blueprint for {}", name);
                println!("{:?}", blueprint);
                for (material, quantity) in materials_for(&blueprint, quantity, &mut eve_db)
                    .map_err(|e| {
                        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
                    })?
                    .into_iter()
                {
                    stack.push((material, quantity));
                }
            }
            Some(blueprint) => {
                println!("Could not find blueprint {}", blueprint.type_id);
                result
                    .entry(name)
                    .and_modify(|e| *e += quantity)
                    .or_insert(quantity);
            }
            None => {
                result
                    .entry(name)
                    .and_modify(|e| *e += quantity)
                    .or_insert(quantity);
            }
        }
    }

    let mut writer = csv::WriterBuilder::new()
        .delimiter(b'\t')
        .from_writer(vec![]);

    let mut keys: Vec<_> = result.keys().cloned().collect();
    keys.sort();

    for k in keys.iter() {
        writer
            .serialize(result.get_key_value(k).unwrap())
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response())?;
    }

    let result = String::from_utf8(
        writer
            .into_inner()
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response())?,
    )
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response())?;

    Ok(result)
}

async fn register_character(
    State(state): State<Arc<AppState>>,
    mut session: WritableSession,
) -> Result<Redirect, Response> {
    let config = &state.config;

    let esi = esi_client(config)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response())?;

    let (url, state) = esi
        .get_authorize_url()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response())?;

    session
        .insert("state", state)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response())?;

    Ok(Redirect::to(&url))
}

fn esi_client(config: &Config) -> Result<Esi> {
    Ok(EsiBuilder::new()
        .user_agent(USER_AGENT)
        .client_id(&config.client_id)
        .client_secret(&config.client_secret)
        .callback_url(&config.redirect_url)
        .scope(&config.scopes)
        .build()?)
}

async fn oauth(
    Query(redirect): Query<EveRedirectRequest>,
    State(state): State<Arc<AppState>>,
    mut session: WritableSession,
) -> Result<String, Response> {
    use crate::schema::state::characters::dsl::*;

    let config = &state.config;
    let mut state_db = state.state_db.lock().await;

    let state: String = session.get("state").ok_or_else(|| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Could not determine oauth flow state",
        )
            .into_response()
    })?;

    session.remove("state");

    if state != redirect.state {
        return Err(StatusCode::BAD_REQUEST.into_response());
    }

    let mut esi = esi_client(config)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response())?;

    esi.authenticate(&redirect.code)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response())?;

    let character = verify_character(esi.access_token.clone().unwrap())
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response())?;

    let character = Character {
        id: character.character_id.clone(),
        name: character.character_name.clone(),
        access_token: esi.access_token.unwrap().clone(),
        refresh_token: esi.refresh_token.unwrap().clone(),
        expiry: NaiveDateTime::from_timestamp_millis(esi.access_expiration.unwrap().clone() as i64)
            .unwrap(),
    };

    diesel::insert_into(characters)
        .values(&character)
        .on_conflict(id)
        .do_update()
        .set(&character)
        .execute(&mut *state_db)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response())?;

    Ok(format!(
        "Character {} ({}) registered",
        &character.name, &character.id
    ))
}

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::new()?;
    let eve_db = Mutex::new(PgConnection::establish(&config.database_url)?);
    let state_db = Mutex::new(state_db()?);
    let state = Arc::new(AppState {
        config,
        eve_db,
        state_db,
    });

    let store = MemoryStore::new();
    let mut secret: [u8; 128] = [0; 128];
    thread_rng().fill_bytes(&mut secret);
    let session_layer = SessionLayer::new(store, &secret).with_same_site_policy(SameSite::Lax);
    let tracing_layer = TraceLayer::new_for_http();

    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/register", get(register_character))
        .route("/oauth-callback", get(oauth))
        .route("/query", get(query))
        .with_state(state)
        .layer(
            ServiceBuilder::new()
                .layer(tracing_layer)
                .layer(session_layer),
        );

    Server::bind(&"127.0.0.1:1337".parse()?)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
