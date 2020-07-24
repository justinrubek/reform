use std::collections::HashMap;

use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct SchemaInfo {
    pub id: u32,
    pub data: serde_json::Value,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SchemaInfoWrapper {
    pub schema: SchemaInfo,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SchemaCreateInfo {
    pub data: serde_json::Value,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SchemaCreateInfoWrapper {
    pub schema: SchemaCreateInfo,
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

pub type DeleteWrapper = HashMap<(), ()>;use crate::error::Error;

