use std::io;
use std::{collections::HashMap, sync::Arc};

use axum::http::StatusCode;
use axum::{
    extract::{Query, State},
    routing::get,
    Router, Server,
};
use diesel::prelude::*;
use diesel::PgConnection;
use eyre::Result;
use hyper::Method;
use hyper::{body::Buf, Body, Client, Request};
use hyper_tls::HttpsConnector;
use rfesi::prelude::*;
use serde::Deserialize;
use serde::Serialize;
use tokio::sync::mpsc::{self, Sender};
use tokio::sync::Mutex;

use crate::config::Config;
use crate::models::*;

mod config;
pub mod models;
pub mod schema;

const VERIFY_URL: &str = "https://login.eveonline.com/oauth/verify";
const USER_AGENT: &str = "economeve/1.0";
const JACKDAW: &str = "Jackdaw";

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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(dead_code)]
struct VerifiedCharacter {
    #[serde(rename = "CharacterID")]
    pub character_id: u64,
    pub character_name: String,
    pub expires_on: String,
    pub scopes: String,
    pub token_type: String,
    pub character_owner_hash: String,
    pub intellectual_property: String,
}

fn blueprint_for(inv_type: &InvType, conn: &mut PgConnection) -> Option<IndustryActivityProduct> {
    println!("Looking up blueprint for {:?}", inv_type.type_name);
    IndustryActivityProduct::belonging_to(&inv_type)
        .select(IndustryActivityProduct::as_select())
        .get_result(conn)
        .ok()
}

fn materials_for(
    blueprint: &IndustryActivityProduct,
    quantity: i32,
    conn: &mut PgConnection,
) -> Result<Vec<(InvType, i32)>> {
    use self::schema::industry_activity_materials;
    use self::schema::inv_types;

    let quantity = (quantity as f32 / blueprint.quantity as f32).ceil() as i32;

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
                .map(|r| (r, quantity * material.quantity))
        })
        .collect::<std::result::Result<Vec<_>, diesel::result::Error>>()?)
}

async fn oauth(
    Query(redirect): Query<EveRedirectRequest>,
    State(state): State<ServeState>,
) -> Result<&'static str, StatusCode> {
    let ServeState { state, tx } = state;
    assert_eq!(state.as_str(), redirect.state.as_str());
    tx.lock()
        .await
        .send(redirect.code.clone())
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok("Success")
}

#[derive(Clone)]
struct ServeState {
    state: Arc<String>,
    tx: Arc<Mutex<Sender<String>>>,
}

async fn serve(state: String, tx: Sender<String>) -> Result<()> {
    let state = Arc::new(state);
    let tx = Arc::new(Mutex::new(tx));
    let app = Router::new()
        .route("/oauth-callback", get(oauth))
        .with_state(ServeState { state, tx });

    println!("about to run server");
    Server::bind(&"127.0.0.1:1337".parse()?)
        .serve(app.into_make_service())
        .await?;

    Ok(())
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

async fn esi_client(config: &Config) -> Result<Esi> {
    let mut esi = EsiBuilder::new()
        .user_agent(USER_AGENT)
        .client_id(&config.client_id)
        .client_secret(&config.client_secret)
        .callback_url(&config.redirect_url)
        .scope(&config.scopes)
        .build()?;

    let (url, state) = esi.get_authorize_url()?;

    let (tx, mut rx) = mpsc::channel(1);

    let join = tokio::spawn(async move {
        let _ = serve(state, tx).await;
    });

    println!("please open {}", url);
    open::that(url)?;
    let code = rx.recv().await.unwrap();

    esi.authenticate(&code).await?;
    esi.update_spec().await?;
    join.abort();

    Ok(esi)
}

#[tokio::main]
async fn main() -> Result<()> {
    use self::schema::inv_types;

    let config = Config::new()?;
    let conn = &mut PgConnection::establish(&config.database_url)?;
    let esi = esi_client(&config).await?;

    println!("authenticated");

    let character = verify_character(esi.access_token.clone().unwrap()).await?;

    println!("{:?}", character);

    let assets: Vec<_> = esi
        .group_assets()
        .get_character_assets(character.character_id)
        .await?
        .iter()
        .map(|asset| asset.type_id)
        .collect();

    println!("{:?}", assets);

    let root = inv_types::table
        .filter(inv_types::type_name.eq(JACKDAW))
        .select(InvType::as_select())
        .get_result(conn)?;

    let mut stack = vec![(root, 1)];
    let mut result = HashMap::new();

    while let Some((current, quantity)) = stack.pop() {
        let name = current.type_name.clone().unwrap();

        match blueprint_for(&current, conn) {
            Some(blueprint) if assets.contains(&(blueprint.type_id as u64)) => {
                println!("Found blueprint for {}", name);
                for (material, quantity) in materials_for(&blueprint, quantity, conn)?.into_iter() {
                    stack.push((material, quantity));
                }
            }
            Some(_) | None => {
                result
                    .entry(name)
                    .and_modify(|e| *e += quantity)
                    .or_insert(quantity);
            }
        }
    }

    let mut writer = csv::Writer::from_writer(io::stdout());

    let mut keys: Vec<_> = result.keys().cloned().collect();
    keys.sort();

    for k in keys.iter() {
        writer.serialize(result.get_key_value(k).unwrap())?;
    }
    // for material in result.iter() {
    //     writer.serialize(material)?;
    // }

    writer.flush()?;

    Ok(())
}
