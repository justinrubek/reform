use crate::auth_agent::{Requests, limit};
use crate::error::Error;
use crate::types::*;

use yew::callback::Callback;
use yew::format::{Json, Nothing, Text};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::services::storage::{Area, StorageService};

#[derive(Default, Debug)]
pub struct Form {
    requests: Requests,
}

pub const API_KEY: &'static str = "forms";

impl Form {
    pub fn new() -> Self {
        Self {
            requests: Requests::new(),
        }
    }

    pub fn get(
        &mut self,
        id: u32,
        callback: Callback<Result<FormInfo, Error>>,
    ) -> FetchTask {
        self.requests
            .get::<FormInfo>(
                format!("/{}/{}", API_KEY, id), 
                callback
            )
    }

    pub fn get_all(
        &mut self,
        callback: Callback<Result<Vec<FormInfo>, Error>>,
    ) -> FetchTask {
        self.requests
            .get::<Vec<FormInfo>>(
                format!("/{}", API_KEY), 
                callback
            )
    }

    pub fn create(
        &mut self,
        form: FormCreateInfo,
        callback: Callback<Result<FormInfo, Error>>,
    ) -> FetchTask {
        self.requests
            .post::<FormCreateInfo, FormInfo>(
                format!("/{}", API_KEY),
                form,
                callback,
            )
    }
}
