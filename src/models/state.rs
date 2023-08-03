use chrono::prelude::*;
use diesel::prelude::*;

use crate::schema::state as schema;

#[derive(Identifiable, Queryable, Selectable, Insertable, AsChangeset)]
#[diesel(table_name = schema::characters)]
pub struct Character {
    pub id: i32,
    pub name: String,
    pub access_token: String,
    pub refresh_token: String,
    pub expiry: NaiveDateTime,
}
