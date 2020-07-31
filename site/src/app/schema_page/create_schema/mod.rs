use std::collections::HashMap;

use yew::format::Json;
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Response, Request};

use crate::auth_agent;
use crate::auth_agent::Schema;
use crate::error::Error;
use crate::types::{SchemaCreateInfo, SchemaInfo};


mod field;
use field::{Field, FieldItem, Type};

pub struct CreateSchema {
    state: State,
    link: ComponentLink<Self>,
    onback: Option<Callback<()>>,
    api_handler: Schema,
    task: Option<FetchTask>,
}

#[derive(Default)]
struct State {
    fields: Vec<Field>,
}

pub enum Msg {
    UpdateField(usize, Field),
    AddField,
    PostSchema,
    CreateSchemaSuccess(SchemaInfo),
    CreateSchemaFailure,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub onback: Option<Callback<()>>,
}


impl Component for CreateSchema {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        CreateSchema { 
            state: Default::default(),
            link,
            onback: props.onback,
            api_handler: auth_agent::Schema::new(),
            task: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateField(i, field) => {
                self.state.fields[i] = field;
                true
            }
            Msg::AddField => {
                self.state.fields.push(Default::default()); 
                true
            }
            Msg::CreateSchemaSuccess(schema_info) => {
                info!("Create schema success: {},{}", schema_info.id, schema_info.data);
                self.task = None;
                false
            }
            Msg::CreateSchemaFailure => {
                warn!("Failure to create schema");
                self.task = None;
                true
            }
            Msg::PostSchema => {
                // TODO: Disable the login button to prevent duplicate reuqests

                // Turn this schema into JSON value
                // required - what fields belong on the document
                let required = format!("{:?}", self.state.fields.iter().map(|field| field.name.clone()).collect::<Vec<String>>());

                fn type_to_string(ty: Type) -> &'static str {
                    match ty {
                        Type::Number => "number",
                        Type::Text => "string",
                    }
                }

                // properties - describe the properties of the fields
                let mut properties = HashMap::new();
                for field in &self.state.fields {
                    properties.insert(field.name.clone(), format!(r#"{{ "type": "{}" }}"#, type_to_string(field.ftype)));
                }

                let schema_data = json!({
                    "type": "object",
                    "required": required,
                    "properties": properties
                });
                let schema_info = SchemaCreateInfo {
                    data: schema_data,
                };

                

                self.task = Some(self.api_handler.create(schema_info, self.link.callback(move |response: Result<SchemaInfo, Error>| {
                    debug!("Response received for CreateSchema");
                    // let (meta, result) = response.into_parts();
                    //
                    //if meta.status.is_success() {
                    if response.is_ok() {
                        Msg::CreateSchemaSuccess(response.unwrap())
                    } else {
                        warn!("{:?}", response.err());
                        Msg::CreateSchemaFailure
                    }
                })));

                true
            }
            _ => false
        }
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
                <h1 class="title">{"Schema creator"}</h1>
                <div class="container">
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
                <button onclick=self.link.callback(|_| Msg::PostSchema) >{"Create schema"}</button>
                </div>
            </>
        }
    }
}

