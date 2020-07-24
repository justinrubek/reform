use crate::auth::Auth;
use crate::config::AppState;
use crate::db::{self};
use crate::errors::{Errors, FieldValidator};

use rocket::State;
use rocket_contrib::json::{Json, JsonValue};
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize)]
pub struct NewSchema {
    schema: NewSchemaData,
}

// TODO: See if we need to validate json schema here
#[derive(Deserialize, Validate)]
struct NewSchemaData {
    data: Option<serde_json::Value>,
}

#[post("/schemas", format = "json", data = "<new_schema>")]
pub fn post_schema(
    new_schema: Json<NewSchema>,
    conn: db::Conn,
    state: State<AppState>,
) -> Result<JsonValue, Errors> {
    let new_schema = new_schema.into_inner().schema;

    let mut extractor = FieldValidator::validate(&new_schema);
    let data = extractor.extract("data", new_schema.data);

    extractor.check()?;

    db::schemas::create(&conn, &data)
        .map(|schema| json!(schema))
        .map_err(|error| {
            Errors::new(&[("json schema", "invalid")])
        })
}

#[get("/schemas/<id>")]
pub fn get_schema(id: i32, auth: Auth, conn: db::Conn, state: State<AppState>) -> Option<JsonValue> {
    db::schemas::find(&conn, id).map(|schema| json!(schema))
}

#[get("/schemas")]
pub fn get_all_schemas(auth: Auth, conn: db::Conn, state: State<AppState>) -> Option<JsonValue> {
    Some(json!(db::schemas::find_all(&conn)))
}

#[get("/schemas/<id>/entries")]
pub fn get_schema_entries(id: i32, auth: Auth, conn: db::Conn, state: State<AppState>) -> Option<JsonValue> {
    println!("id {}", id);
    db::entries::find_by_schema_id(&conn, id).map(|entry| json!(entry))
}

/*
 * Updating of schemas is currently disabled, json schemas will remain persistent
 *
#[derive(Deserialize)]
pub struct UpdateSchema {
    schema: db::schemas::UpdateSchemaData,
}

#[put("/schema/<id>", format = "json", data = "<schema>")]
pub fn put_schema(
    id: i32,
    schema: Json<UpdateSchema>,
    auth: Auth,
    conn: db::Conn,
    state: State<AppState>,
) -> Option<JsonValue> {
    db::schemas::update(&conn, id, &schema.schema).map(|schema| json!(schema))
}
*/
