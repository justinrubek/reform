use crate::auth::Auth;
use crate::config::AppState;
use crate::db::{self};
use crate::errors::{Errors, FieldValidator};

use rocket::State;
use rocket_contrib::json::{Json, JsonValue};
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct NewEntry {
    pub schema_id: Option<i32>,
    pub data: Option<serde_json::Value>,
}

#[post("/entries", format = "json", data = "<new_entry>")]
pub fn post_entries(
    new_entry: Json<NewEntry>,
    conn: db::Conn,
    state: State<AppState>,
) -> Result<JsonValue, Errors> {
    let new_entry = new_entry.into_inner();

    let mut extractor = FieldValidator::validate(&new_entry);
    let schema_id = extractor.extract("schema_id", new_entry.schema_id);
    let data = extractor.extract("data", new_entry.data);

    extractor.check()?;

    // TODO: Validate against schema

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
