use std::collections::HashMap;

use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};

use crate::api;
use crate::components::Select;
use crate::error::Error;
use crate::types::SchemaInfo;

mod field_selector;
use field_selector::FieldSelector;

#[derive(Clone, Default, PartialEq, Serialize)]
pub struct Mapping {
    schema_id: u32,
    field_mappings: HashMap<String, String>,
}

#[derive(Default)]
struct State {
    mapping: Mapping,
    schemas: Vec<SchemaInfo>,
    selected_schema: Option<SchemaInfo>,
}

pub enum Msg {
    OnChange,
    FetchSuccess(Vec<SchemaInfo>),
    FetchFailure(Error),
    SelectSchema(SchemaInfo),
    UpdateMapping(String, String),
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub onchange: Callback<Mapping>,
    pub mapping: Mapping,
}

pub struct MappingItem {
    state: State,
    link: ComponentLink<Self>,
    fetch: api::Schema,
    task: Option<FetchTask>,
    props: Props,
}

// The format of our schema so that we may read the field names
#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct json_schema_form {
    properties: HashMap<String, serde_json::Value>,
    required: serde_json::Value,
}

impl ToString for SchemaInfo {
    fn to_string(&self) -> String {
        format!("{}({})", self.name, self.id)
    }
}

impl Component for MappingItem {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        // Attempt to fetch schemas
        let mut fetch = api::Schema::new();
        let task =
            fetch.get_all(link.callback(
                |response: Result<Vec<SchemaInfo>, Error>| match response {
                    Ok(list) => Msg::FetchSuccess(list),
                    Err(err) => Msg::FetchFailure(err),
                },
            ));

        let state = State {
            mapping: props.mapping.clone(),
            schemas: Vec::new(),
            selected_schema: None,
        };

        MappingItem {
            state,
            link,
            fetch,
            task: Some(task),
            props,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::OnChange => {
                self.props.onchange.emit(self.state.mapping.clone());
                true
            }
            Msg::FetchSuccess(data) => {
                debug!("Success - Schema");
                self.state.schemas = data;
                self.task = None;
                true
            }
            Msg::FetchFailure(error) => {
                self.task = None;
                // TODO: Respond to this
                error!("error: {}", error);
                true
            }
            Msg::SelectSchema(schema_info) => {
                // TODO: Create the full url endpoint (for now, use the relative one)
                self.state.mapping.schema_id = schema_info.id;

                // Add the fields we can map
                self.state.mapping.field_mappings = HashMap::new();

                self.state.selected_schema = Some(schema_info);

                self.link.send_message(Msg::OnChange);
                true
            }
            Msg::UpdateMapping(to_field, from_field) => {
                self.state
                    .mapping
                    .field_mappings
                    .insert(to_field, from_field);
                self.link.send_message(Msg::OnChange);
                true
            }
            _ => false,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let schema_selection = html! {
            <Select<SchemaInfo> options=&self.state.schemas onchange=self.link.callback(|schema| Msg::SelectSchema(schema)) />
        };

        // Build a list of the fields in the schema
        let field_display = match &self.state.selected_schema {
            Some(schema_info) => {
                let schema: json_schema_form = serde_json::from_value(schema_info.data.clone())
                    .expect("schema has extra properties");

                let fields = schema.properties.iter().map(|(name, properties)| {
                    if let serde_json::Value::Object(map) = properties {

                        // Extract the field name and type to pass as a prop
                        let ftype = map.get("type").unwrap_or_else(|| panic!("No type provided for schema field {}", name)).as_str().expect("Failed to get type as str");

                        html! {
                            <tr>
                                <FieldSelector name=name.clone() ftype=ftype.clone() onchange=self.link.callback(|(to, from)| Msg::UpdateMapping(to, from)) />
                            </tr>
                        }
                    } else {
                        panic!("schema field properties is not an object");
                    }
                }).collect::<Html>();

                html! {
                    {fields}
                }
            }
            None => html! {},
        };

        let field_entry = html! {
            <table class="table">
                <thead>
                    <tr>
                        <th>{"name"}</th>
                        <th>{"type"}</th>
                        <th>{"form field to map"}</th>
                    </tr>
                </thead>
                {field_display}
            </table>
        };

        html! {
            <div>
                <label class="label">{"Schema to map to:"}</label>
                {schema_selection}
                {field_entry}
            </div>
        }
    }
}
