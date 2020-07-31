use crate::auth_agent::{Requests, limit};
use crate::error::Error;
use crate::types::*;

use yew::callback::Callback;
use yew::format::{Json, Nothing, Text};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::services::storage::{Area, StorageService};

#[derive(Default, Debug)]
pub struct Schema {
    requests: Requests,
}

pub const API_KEY: &'static str = "schemas";

impl Schema {
    pub fn new() -> Self {
        Self {
            requests: Requests::new(),
        }
    }

    pub fn get(
        &mut self,
        id: u32,
        callback: Callback<Result<SchemaInfo, Error>>,
    ) -> FetchTask {
        self.requests
            .get::<SchemaInfo>(
                format!("/{}/{}", API_KEY, id), 
                callback
            )
    }

    pub fn get_all(
        &mut self,
        callback: Callback<Result<Vec<SchemaInfo>, Error>>,
    ) -> FetchTask {
        self.requests
            .get::<Vec<SchemaInfo>>(
                format!("/{}", API_KEY), 
                callback
            )
    }

    pub fn create(
        &mut self,
        schema: SchemaCreateInfo,
        callback: Callback<Result<SchemaInfo, Error>>,
    ) -> FetchTask {
        self.requests
            .post::<SchemaCreateInfo, SchemaInfo>(
                format!("/{}", API_KEY),
                schema,
                callback,
            )
    }
}
