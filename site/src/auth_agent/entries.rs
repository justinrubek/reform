use crate::auth_agent::{Requests, limit};
use crate::error::Error;
use crate::types::*;

use yew::callback::Callback;
use yew::format::{Json, Nothing, Text};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::services::storage::{Area, StorageService};

#[derive(Default, Debug)]
pub struct Entry {
    requests: Requests,
}

const API_KEY: &'static str = "entries";

impl Entry {
    pub fn new() -> Self {
        Self {
            requests: Requests::new(),
        }
    }

    pub fn get(
        &mut self,
        id: u32,
        callback: Callback<Result<EntryInfoWrapper, Error>>,
    ) -> FetchTask {
        self.requests
            .get::<EntryInfoWrapper>(
                format!("/{}/{}", API_KEY, id), 
                callback
            )
    }

    pub fn get_by_schema_id(
        &mut self,
        id: u32,
        callback: Callback<Result<Vec<EntryInfo>, Error>>,
    ) -> FetchTask {
        self.requests
            .get::<Vec<EntryInfo>>(
                format!("/{}/{}/entries", crate::auth_agent::schemas::API_KEY, id), 
                callback
            )
    }

    pub fn create(
        &mut self,
        entry: EntryCreateInfoWrapper,
        callback: Callback<Result<EntryInfoWrapper, Error>>,
    ) -> FetchTask {
        self.requests
            .post::<EntryCreateInfoWrapper, EntryInfoWrapper>(
                format!("/{}", API_KEY),
                entry,
                callback,
            )
    }
}
