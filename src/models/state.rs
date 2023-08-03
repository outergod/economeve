use chrono::prelude::*;
use diesel::prelude::*;

use crate::schema::state as schema;

#[derive(Queryable, Selectable, Insertable, AsChangeset)]
#[diesel(table_name = schema::characters)]
pub struct Character {
    pub id: i32,
    pub name: String,
}

#[derive(Queryable, Selectable, Insertable, AsChangeset, Associations)]
#[diesel(table_name = schema::tokens)]
#[diesel(belongs_to(Character))]
pub struct Token {
    pub character_id: i32,
    pub access_token: String,
    pub refresh_token: String,
    pub expiry: NaiveDateTime,
}
