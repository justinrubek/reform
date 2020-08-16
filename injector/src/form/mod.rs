use failure::Error;

use std::collections::HashMap;

use yew::format::{Json, Nothing};
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};

mod field;
use field::{Field, FormField, Type};

#[derive(Clone, PartialEq, Deserialize)]
struct Mapping {
    schema_id: i32,
    field_mappings: HashMap<String, String>,
}

#[derive(Clone, PartialEq, Deserialize)]
struct FormInfo {
    id: u32,
    name: String,
    fields: Vec<FormField>,
    mappings: Vec<Mapping>,
}

pub struct Form {
    link: ComponentLink<Self>,
    state: State,
    task: Option<FetchTask>,
    fetch: FetchService,
    submit_tasks: Vec<FetchTask>,
}

#[derive(Debug)]
struct FieldData {
    name: String,
    data: serde_json::Value,
}

impl ToString for FieldData {
    fn to_string(&self) -> String {
        format!("{{name:{},data:{}}}", self.name, self.data)
    }
}

struct State {
    form: Option<FormInfo>,
    field_data: Vec<FieldData>,
}

pub enum Msg {
    GetFormSuccess(FormInfo),
    GetFormFailure,
    UpdateField(usize, serde_json::Value),
    Submit,
    SubmitSuccess,
    SubmitFailure,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub form_url: String,
}

impl Component for Form {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        // Initiate request to retrieve form
        let mut fetch = FetchService::new();
        let request = Request::get(props.form_url).body(Nothing).expect("Failed to build request for form");
        let task = fetch.fetch(request, link.callback(|response: Response<Result<String, failure::Error>>| {
            if response.status().is_success() {
                let body = response.body().as_ref().unwrap();
                let form_info: FormInfo = serde_json::from_str(&body).expect("failed to deserialize form");
                Msg::GetFormSuccess(form_info)
            } else {
                Msg::GetFormFailure
            }
        }));

        let state = State { 
            form: None,
            field_data: Vec::new(),
        };

        Form {
            link,
            state,
            task: Some(task),
            fetch: fetch,
            submit_tasks: Vec::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        // TODO: Disable/enable button as needed
        match msg {
            Msg::GetFormSuccess(form_info) => {
                // Create empty data for each field
                let mut field_data = Vec::new();
                form_info.fields.iter().for_each(|field| {
                    let initial_data = match field.ftype {
                        field::Type::Number => json!(0),
                        field::Type::Text => json!(""),
                        field::Type::Choice => json!(""),
                    };

                    field_data.push(FieldData {
                        name: field.name.clone(),
                        data: initial_data,
                    })
                });

                self.state.form = Some(form_info);
                self.state.field_data = field_data;
                
                info!("form get success");
                true
            }
            Msg::GetFormFailure => {
                true
            }
            Msg::UpdateField(index, data) => {
                self.state.field_data[index].data = data;
                true
            }
            Msg::Submit => {
                #[derive(Clone, PartialEq, Serialize)]
                struct Entry {
                    schema_id: i32,
                    data: serde_json::Value
                }

                let mut fetch = FetchService::new();
                let mappings = self.state.form.clone().unwrap().mappings.clone();
                for mapping in &mappings {
                    // Retrieve the values to be filled into the schema slots
                    let mut fields: HashMap<String, serde_json::Value> = HashMap::new();
                    for (from, to) in &mapping.field_mappings {
                        // Retrieve the value of 'from' from the form and apply to schema name 'to'
                        let value: serde_json::Value = {
                            let mut val: serde_json::Value = "null".into();
                            info!(" field_data{:?}", self.state.field_data);
                            for data in self.state.field_data.iter() {
                                if data.name.eq(from) {
                                    val = data.data.clone();
                                }

                            }
                            val
                        };
                        // let value = self.state.field_data.iter().find(|data| data.name.eq(from)).unwrap().data.clone();
                        fields.insert(to.to_string(), value);
                    }

                    let entry = Entry {
                        schema_id: mapping.schema_id,
                        data: json!(fields),
                    };

                    let body = json!(entry);

                    let request = Request::post("/api/entries")
                        .header("Content-Type", "application/json")
                        .body(Json(&body)).expect("Failed to build request for form submission");
                    let task = fetch.fetch(request, self.link.callback(|response: Response<Result<String, failure::Error>>| {
                        if response.status().is_success() {
                            Msg::SubmitSuccess
                        } else {
                            Msg::SubmitFailure
                        }
                    }));

                    self.submit_tasks.push(task);

                }
                true
            }
            Msg::SubmitSuccess => {
                info!("Successfully submit form");
                self.submit_tasks.clear();
                true
            }
            Msg::SubmitFailure => {
                info!("Failed to submit form");
                self.submit_tasks.clear();
                true
            }
            _ => true
        }
    }

    fn view(&self) -> Html {
        let fields = match &self.state.form {
            Some(form) => {
                form.fields.iter().enumerate().map(|(i, field)| {
                    let onchange = self.link.callback(move |data| Msg::UpdateField(i, data));
                    html! {
                        <Field field=field onchange=onchange value=self.state.field_data[i].data.clone() />
                    }
                }).collect::<Html>()
            }
            None => html!{}
        };

        let submit = match &self.state.form {
            Some(form) => {
                html! {
                    <button class="button" onclick=self.link.callback(|_| Msg::Submit)>{"Submit"}</button>
                }
            }
            None => html!{}
        };

        html! {
            <div>
                <div>
                    {fields}
                </div>
                {submit}
            </div>
        }
    }
}
