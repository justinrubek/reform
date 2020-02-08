#[macro_use]
extern crate log;

#[macro_use]
extern crate serde;

extern crate serde_json;

#[macro_use]
extern crate stdweb;

extern crate web_logger;

mod app;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    web_logger::init();
    info!("Yew app startup");
    
    yew::start_app::<app::App>();

    Ok(())
}
