use serde::Serialize;

use crate::models::schema::Schema;
use crate::schema::entries;

#[derive(Associations, Identifiable, PartialEq, Queryable, Serialize)]
#[belongs_to(Schema)]
#[table_name = "entries"]
pub struct Entry {
    pub id: i32,
    pub schema_id: i32,
    pub data: serde_json::Value,
}
