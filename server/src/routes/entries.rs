use crate::auth::Auth;
use crate::config::AppState;
use crate::db::{self};
use crate::errors::{Errors, FieldValidator};

use core::ops::Deref;
use std::convert::{From, Into};
use rocket::State;
use rocket_contrib::json::{Json, JsonValue};
use serde::{Serialize, Deserialize};
use serde_json::Value;
use validator::Validate;
use valico::json_schema;

#[derive(Deserialize, Validate, Serialize)]
pub struct NewEntry {
    pub schema_id: Option<i32>,
    pub data: Option<serde_json::Value>,
}

#[post("/entries", format = "json", data = "<new_entry_json>")]
pub fn post_entries(
    new_entry_json: Json<NewEntry>,
    conn: db::Conn,
    state: State<AppState>,
) -> Result<JsonValue, Errors> {
    let new_entry = new_entry_json.into_inner();

    let mut extractor = FieldValidator::validate(&new_entry);
    let schema_id = extractor.extract("schema_id", new_entry.schema_id);
    let data = extractor.extract("data", new_entry.data.clone());

    extractor.check()?;

    // Validate against schema
    // First, prepare the schema into the jsl format
    let schema = db::schemas::find(&conn, schema_id).ok_or(Errors::new(&[("schema_id", "invalid")]))?.data;

    let mut scope = json_schema::Scope::new();
    let schema_validator = scope.compile_and_return(schema.clone(), false).unwrap();

    let validation = schema_validator.validate(&data);
    if validation.is_valid() == false {
        let errors = validation.errors;
        println!("{:?}", errors);
        return Err(Errors::new(&[("json_entry", "invalid")]));
    }

    db::entries::create(&conn, &schema_id, &data)
        .map(|entry| json!(entry))
        .map_err(|error| {
            Errors::new(&[("json entry", "invalid")])
        })
}

#[get("/entry/<id>")]
pub fn get_entry(id: i32, auth: Auth, conn: db::Conn, state: State<AppState>) -> Option<JsonValue> {
    db::entries::find(&conn, id).map(|entry| json!(entry))
}

#[derive(Deserialize)]
pub struct UpdateEntry {
    entry: db::entries::UpdateEntryData,
}

#[put("/entry/<id>", format = "json", data = "<entry>")]
pub fn put_entry(
    id: i32,
    entry: Json<UpdateEntry>,
    auth: Auth,
    conn: db::Conn,
    state: State<AppState>,
) -> Option<JsonValue> {
    db::entries::update(&conn, id, &entry.entry).map(|entry| json!(entry))
}
