use crate::models::schema::Schema;
use crate::schema::schemas;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::{DatabaseErrorKind, Error};
use serde::Deserialize;

#[derive(Insertable)]
#[table_name = "schemas"]
pub struct NewSchema<'a> {
    pub data: &'a serde_json::Value,
    pub name: &'a str,
}

pub enum SchemaCreationError {
    InvalidSchemaJson,
}

impl From<Error> for SchemaCreationError {
    fn from(err: Error) -> SchemaCreationError {
        // TODO: Apply this error only when relevant
        return SchemaCreationError::InvalidSchemaJson;

        panic!("Error creating schema: {:?}", err)
    }
}

pub fn create(
    conn: &PgConnection,
    data: &serde_json::Value,
    name: &str,
) -> Result<Schema, SchemaCreationError> {
    let new_schema = &NewSchema {
        data,
        name,
    };

    diesel::insert_into(schemas::table)
        .values(new_schema)
        .get_result::<Schema>(conn)
        .map_err(Into::into)
}

pub fn find(conn: &PgConnection, id: i32) -> Option<Schema> {
    schemas::table
        .find(id)
        .get_result(conn)
        .map_err(|err| println!("find_schema: {}", err))
        .ok()
}

pub fn find_all(conn: &PgConnection) -> Option<Vec<Schema>> {
    use crate::schema::schemas::dsl::*;
    schemas.load(conn).ok()
}

#[derive(Deserialize, AsChangeset, Default, Clone)]
#[table_name = "schemas"]
pub struct UpdateSchemaData {
    data: Option<serde_json::Value>,
    name: Option<String>
}

pub fn update(conn: &PgConnection, id: i32, data: &UpdateSchemaData) -> Option<Schema> {
    let data = &UpdateSchemaData {
        ..data.clone()
    };
    diesel::update(schemas::table.find(id))
        .set(data)
        .get_result(conn)
        .ok()
}
