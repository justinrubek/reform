#![recursion_limit="256"]

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
use stdweb::web::{document, Element, IElement, IParentNode, Node};
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
    info!("Form URL: {}", form_url);

    let form_props = form::Props {
        form_url: form_url,
    };
    app.mount_with_props(element, form_props);
    
    Ok(())
}
