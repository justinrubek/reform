use serde::Serialize;

use crate::schema::forms;

#[derive(Identifiable, PartialEq, Queryable, Serialize)]
pub struct Form {
    pub id: i32,
    pub name: String,
    pub fields: serde_json::Value,
}

