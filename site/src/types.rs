use std::collections::HashMap;

use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct SchemaInfo {
    pub id: u32,
    pub data: serde_json::Value,
    pub name: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SchemaCreateInfo {
    pub data: serde_json::Value,
    pub name: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SchemaList {
    pub schemas: Vec<SchemaInfo>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct EntryInfo {
    pub id: u32,
    pub schema_id: u32,
    pub data: serde_json::Value,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EntryInfoWrapper {
    pub entry: EntryInfo,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EntryCreateInfo {
    pub schema_id: u32,
    pub data: serde_json::Value,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EntryCreateInfoWrapper {
    pub entry: EntryCreateInfo,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EntryList {
    pub entries: Vec<EntryInfo>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct FormInfo {
    pub id: u32,
    pub name: String,
    pub fields: serde_json::Value,
    pub mappings: serde_json::Value,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FormCreateInfo {
    pub name: String,
    pub fields: serde_json::Value,
    pub mappings: serde_json::Value,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FormList {
    pub forms: Vec<FormInfo>,
}

pub type DeleteWrapper = HashMap<(), ()>;use crate::error::Error;

