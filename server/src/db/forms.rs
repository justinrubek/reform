use crate::models::form::Form;
use crate::schema::forms;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::{DatabaseErrorKind, Error};
use serde::Deserialize;

#[derive(Insertable)]
#[table_name = "forms"]
pub struct NewForm<'a> {
    pub name:  &'a str,
    pub fields: &'a serde_json::Value,
}

pub enum FormCreationError {
    InvalidFormJson,
}

impl From<Error> for FormCreationError {
    fn from(err: Error) -> FormCreationError {
        // TODO: Apply this error only when relevant
        return FormCreationError::InvalidFormJson;

        panic!("Error creating form: {:?}", err)
    }
}

pub fn create(
    conn: &PgConnection,
    name: &str,
    fields: &serde_json::Value,
) -> Result<Form, FormCreationError> {
    let new_form = &NewForm {
        name: name,
        fields: fields,
    };

    diesel::insert_into(forms::table)
        .values(new_form)
        .get_result::<Form>(conn)
        .map_err(Into::into)
}

pub fn find(conn: &PgConnection, id: i32) -> Option<Form> {
    forms::table
        .find(id)
        .get_result(conn)
        .map_err(|err| println!("find_form: {}", err))
        .ok()
}

pub fn find_all(conn: &PgConnection) -> Option<Vec<Form>> {
    use crate::schema::forms::dsl::*;
    forms.load(conn).ok()
}

#[derive(Deserialize, AsChangeset, Default, Clone)]
#[table_name = "forms"]
pub struct UpdateFormData {
    name: Option<String>,
    fields: Option<serde_json::Value>,
}

pub fn update(conn: &PgConnection, id: i32, data: &UpdateFormData) -> Option<Form> {
    let data = &UpdateFormData {
        ..data.clone()
    };
    diesel::update(forms::table.find(id))
        .set(data)
        .get_result(conn)
        .ok()
}
