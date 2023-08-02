use diesel::prelude::*;

use crate::schema::state as schema;

#[derive(Queryable, Selectable, Insertable, AsChangeset)]
#[diesel(table_name = schema::characters)]
pub struct Character {
    pub id: String,
    pub refresh_token: String,
}
