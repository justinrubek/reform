#![recursion_limit = "256"]

extern crate anyhow;

extern crate chrono;

extern crate dotenv_codegen;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate log;

#[macro_use]
extern crate parking_lot;

#[macro_use]
extern crate serde;

#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate yew;

#[macro_use]
extern crate yew_router;

#[macro_use]
extern crate stdweb;

extern crate web_logger;

pub mod api;
mod app;
pub mod error;
pub mod types;

mod components;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    web_logger::init();
    info!("Yew app startup");

    yew::start_app::<app::App>();

    Ok(())
}
