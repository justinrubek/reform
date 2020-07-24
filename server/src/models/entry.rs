use serde::Serialize;

use crate::schema::entries;
use crate::models::schema::Schema;

#[derive(Associations, Identifiable, PartialEq, Queryable, Serialize)]
#[belongs_to(Schema)]
#[table_name = "entries"]
pub struct Entry {
    pub id: i32,
    pub schema_id: i32,
    pub data: serde_json::Value,
}

