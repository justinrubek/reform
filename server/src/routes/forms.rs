use crate::auth::Auth;
use crate::config::AppState;
use crate::db::{self};
use crate::errors::{Errors, FieldValidator};

use rocket::State;
use rocket_contrib::json::{Json, JsonValue};
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize)]
pub struct NewForm {
    form: NewFormData,
}

#[derive(Deserialize, Validate)]
struct NewFormData {
    name: Option<String>,
    fields: Option<serde_json::Value>,
}

#[post("/forms", format = "json", data = "<new_form>")]
pub fn post_form(
    new_form: Json<NewForm>,
    conn: db::Conn,
    state: State<AppState>,
) -> Result<JsonValue, Errors> {
    let new_form = new_form.into_inner().form;

    let mut extractor = FieldValidator::validate(&new_form);
    let name = extractor.extract("name", new_form.name);
    let data = extractor.extract("fields", new_form.fields);

    extractor.check()?;

    db::forms::create(&conn, &name, &data)
        .map(|form| json!(form))
        .map_err(|error| {
            Errors::new(&[("json form", "invalid")])
        })
}

#[get("/forms/<id>")]
pub fn get_form(id: i32, auth: Auth, conn: db::Conn, state: State<AppState>) -> Option<JsonValue> {
    db::forms::find(&conn, id).map(|form| json!(form))
}

#[get("/forms")]
pub fn get_all_forms(auth: Auth, conn: db::Conn, state: State<AppState>) -> Option<JsonValue> {
    Some(json!(db::forms::find_all(&conn)))
}

