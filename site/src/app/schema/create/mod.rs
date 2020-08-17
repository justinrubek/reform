use std::collections::HashMap;


use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Response, Request};

use crate::api;
use crate::api::Schema;
use crate::error::Error;
use crate::types::{SchemaCreateInfo, SchemaInfo};


mod field;
use field::{Field, FieldItem, Type};

pub struct CreateSchema {
    state: State,
    link: ComponentLink<Self>,
    api_handler: Schema,
    task: Option<FetchTask>,
    message: Html,
}

#[derive(Default)]
struct State {
    fields: Vec<Field>,
    name: String,
}

pub enum Msg {
    UpdateName(String),
    UpdateField(usize, Field),
    AddField,
    PostSchema,
    CreateSchemaSuccess(SchemaInfo),
    CreateSchemaFailure(Error),
    ClearMessage,
}

impl Component for CreateSchema {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        CreateSchema { 
            state: Default::default(),
            link,
            api_handler: api::Schema::new(),
            task: None,
            message: html!{},
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateName(name) => {
                self.state.name = name;
                true
            }
            Msg::UpdateField(i, field) => {
                self.state.fields[i] = field;
                true
            }
            Msg::AddField => {
                self.state.fields.push(Default::default()); 
                true
            }
            Msg::CreateSchemaSuccess(schema_info) => {
                self.task = None;
                self.message = html! {
                    <article class="message is-primary">
                        <div class="message-header">
                            <p>{"Created schema successfully"}</p>
                            <button class="delete" aria-label="delete" onclick=self.link.callback(|_| Msg::ClearMessage)></button>
                        </div>
                        <div class="message-body">
                        {format!("Created schema {} with id {}", schema_info.name, schema_info.id)}
                        </div>
                    </article>
                };
                true
            }
            Msg::CreateSchemaFailure(error) => {
                self.task = None;
                self.message = html! {
                    <article class="message is-danger">
                        <div class="message-header">
                            <p>{"Failed to create schema"}</p>
                            <button class="delete" aria-label="delete" onclick=self.link.callback(|_| Msg::ClearMessage)></button>
                        </div>
                        <div class="message-body">
                        {error}
                        </div>
                    </article>
                };
                true
            }
            Msg::PostSchema => {
                // TODO: Disable the login button to prevent duplicate reuqests

                // Turn this schema into JSON value
                // required - what fields belong on the document
                // let required = format!("{:?}", self.state.fields.iter().map(|field| field.name.clone()).collect::<Vec<String>>());
                let required_fields = self.state.fields.iter().map(|field| json!(field.name)).collect::<Vec<serde_json::Value>>();
                let required = serde_json::Value::Array(required_fields);

                fn type_to_string(ty: Type) -> &'static str {
                    match ty {
                        Type::Number => "number",
                        Type::Text => "string",
                    }
                }

                // properties - describe the properties of the fields
                let mut properties: HashMap<String, serde_json::Value> = HashMap::new();
                for field in &self.state.fields {
                    let mut props: serde_json::Map<String, serde_json::Value> = serde_json::Map::new();
                    props.insert("type".to_string(), json!(type_to_string(field.ftype).to_string()));

                    properties.insert(field.name.clone(), serde_json::Value::Object(props));
                }

                let schema_data = json!({
                    "type": "object",
                    "required": required,
                    "properties": properties
                });

                let schema_info = SchemaCreateInfo {
                    data: schema_data,
                    name: self.state.name.clone(),
                };

                self.task = Some(self.api_handler.create(schema_info, self.link.callback(move |response: Result<SchemaInfo, Error>| {
                    debug!("Response received for CreateSchema");
                    if response.is_ok() {
                        Msg::CreateSchemaSuccess(response.unwrap())
                    } else {
                        Msg::CreateSchemaFailure(response.err().unwrap())
                    }
                })));

                true
            }
            Msg::ClearMessage => {
                self.message = html!{};
                true
            }
            _ => false
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let fields = self.state.fields.iter().enumerate().map(|(i, field)| {
            html! {
                <FieldItem field=field onchange=&self.link.callback(move |field| {
                    Msg::UpdateField(i, field)
                }) />
            }
        }).collect::<Html>();

        html! {
            <>
                {self.message.clone()}
                <h1 class="title">{"Schema creator"}</h1>
                <div class="container">
                    <label class="label" for="schema_name">{"Name"}</label>
                    <input class="input" type="text" name="schema_name" value=self.state.name
                        oninput=self.link.callback(|e: InputData| Msg::UpdateName(e.value))
                    />
                    <div class="media">
                        <h2 class="title media-left">{"Fields"}</h2> 
                        <button class="button media-right" onclick=self.link.callback(|_| Msg::AddField)>{"add field"}</button>
                    </div>
                    <table class="table">
                        <thead>
                            <tr>
                                <th>{"name"}</th>
                                <th>{"type"}</th>
                            </tr>
                        </thead>
                        {fields}
                    </table>
                <button class="button" onclick=self.link.callback(|_| Msg::PostSchema) >{"Create schema"}</button>
                </div>
            </>
        }
    }
}

