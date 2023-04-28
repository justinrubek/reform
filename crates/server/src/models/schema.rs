use serde::Serialize;

use crate::schema::schemas;

#[derive(Identifiable, PartialEq, Queryable, Serialize)]
pub struct Schema {
    pub id: i32,
    pub data: serde_json::Value,
    pub name: String,
}

