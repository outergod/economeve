use std::collections::HashMap;
use std::io;

use diesel::prelude::*;
use diesel::PgConnection;
use eyre::Result;
use serde::Serialize;

use crate::config::Config;
use crate::models::*;

// mod domain;
mod config;
pub mod models;
pub mod schema;

const JACKDAW: &str = "Jackdaw";

#[derive(Debug, Serialize)]
struct Material {
    name: String,
    quantity: i32,
}

fn materials_for(inv_type: &InvType, conn: &mut PgConnection) -> Result<Vec<(InvType, i32)>> {
    use self::schema::inv_types;

    Ok(InvTypeMaterial::belonging_to(&inv_type)
        .select(InvTypeMaterial::as_select())
        .load(conn)?
        .into_iter()
        .map(|material| {
            inv_types::table
                .filter(inv_types::type_id.eq(material.material_type_id))
                .select(InvType::as_select())
                .get_result(conn)
                .map(|r| (r, material.quantity))
        })
        .collect::<std::result::Result<Vec<_>, diesel::result::Error>>()?)
}

fn main() -> Result<()> {
    use self::schema::inv_types;

    let config = Config::new()?;
    let conn = &mut PgConnection::establish(&config.database_url)?;

    let root = inv_types::table
        .filter(inv_types::type_name.eq(JACKDAW))
        .select(InvType::as_select())
        .get_result(conn)?;

    let mut stack = vec![root];
    let mut result = HashMap::new();

    while let Some(current) = stack.pop() {
        let materials = materials_for(&current, conn)?;

        for (material, quantity) in materials.into_iter() {
            if let Some(ref name) = material.type_name {
                if config.blueprints.contains(&name) {
                    stack.push(material);
                } else {
                    result
                        .entry(name.to_string())
                        .and_modify(|e| *e += quantity)
                        .or_insert(quantity);
                }
            }
        }
    }

    let mut writer = csv::Writer::from_writer(io::stdout());

    for material in result.iter() {
        writer.serialize(material)?;
    }

    writer.flush()?;

    Ok(())
}
