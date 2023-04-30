#![recursion_limit = "256"]
extern crate anyhow;

extern crate chrono;

#[macro_use]
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
extern crate stdweb;

extern crate web_logger;

mod form;
use form::Props;

use std::convert::TryInto;

use yew::utils::document;
use yew::web_sys::Element;

use wasm_bindgen::prelude::*;

use crate::form::Form;

#[wasm_bindgen]
pub fn inject_forms() -> Result<(), JsValue> {
    web_logger::init();
    info!("Form injector startup");

    let app: yew::App<Form> = yew::App::new();

    // Find the elements which should be injected
    let element = document().query_selector(".reform-form").unwrap().unwrap();

    // Retrieve properties to pass the field
    let form_url = element.get_attribute("data-form").unwrap();
    let success_message_title = element
        .get_attribute("data-success-message-title")
        .unwrap_or("Submission success!".to_string());
    let success_message_body = element
        .get_attribute("data-success-message-body")
        .unwrap_or("Thank you for your submission. It has been received.".to_string());

    let form_props = form::Props {
        form_url,
        success_message_title,
        success_message_body,
    };
    app.mount_with_props(element, form_props);

    Ok(())
}
