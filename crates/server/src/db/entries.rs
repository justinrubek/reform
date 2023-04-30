use crate::models::entry::Entry;
use crate::schema::{entries, schemas};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::{DatabaseErrorKind, Error};
use serde::Deserialize;

#[derive(Insertable)]
#[table_name = "entries"]
pub struct NewEntry<'a> {
    pub schema_id: &'a i32,
    pub data: &'a serde_json::Value,
}

pub enum EntryCreationError {
    InvalidJson,
}

impl From<Error> for EntryCreationError {
    fn from(err: Error) -> EntryCreationError {
        return EntryCreationError::InvalidJson;
    }
}

pub fn create(
    conn: &PgConnection,
    schema_id: &i32,
    data: &serde_json::Value,
) -> Result<Entry, EntryCreationError> {
    let new_entry = &NewEntry { schema_id, data };

    // TODO: Query the schema and ensure the provided JSON is valid

    diesel::insert_into(entries::table)
        .values(new_entry)
        .get_result::<Entry>(conn)
        .map_err(Into::into)
}

pub fn find(conn: &PgConnection, id: i32) -> Option<Entry> {
    entries::table
        .find(id)
        .get_result(conn)
        .map_err(|err| println!("find_user: {}", err))
        .ok()
}

pub fn find_by_schema_id(conn: &PgConnection, id: i32) -> Option<Vec<Entry>> {
    // Get the schema we're querying for
    match crate::db::schemas::find(conn, id) {
        Some(schema) => Entry::belonging_to(&schema).load(conn).ok(),
        _ => None,
    }
}

#[derive(Deserialize, AsChangeset, Default, Clone)]
#[table_name = "entries"]
pub struct UpdateEntryData {
    schema_id: Option<i32>,
    data: Option<serde_json::Value>,
}

pub fn update(conn: &PgConnection, id: i32, data: &UpdateEntryData) -> Option<Entry> {
    let data = &UpdateEntryData { ..data.clone() };
    diesel::update(entries::table.find(id))
        .set(data)
        .get_result(conn)
        .ok()
}
