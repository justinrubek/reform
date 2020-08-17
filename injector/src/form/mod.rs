use anyhow::Error;

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
    submit_tasks: Vec<FetchTask>,
    submit_completion: Vec<bool>,
    message: Html,
    props: Props,
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
    GetFormFailure(String),
    UpdateField(usize, serde_json::Value),
    Submit,
    SubmitSuccess(usize),
    SubmitFailure(String),
    ClearMessage,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub form_url: String,
    #[prop_or("Submission success!".to_string())]
    pub success_message_title: String,
    #[prop_or("Thank you for your submission. It has been received.".to_string())]
    pub success_message_body: String,
}

impl Component for Form {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        // Initiate request to retrieve form
        let request = Request::get(props.form_url.clone()).body(Nothing).expect("Failed to build request for form");
        let task = FetchService::fetch(request, link.callback(|response: Response<Result<String, anyhow::Error>>| {
            if response.status().is_success() {
                let body = response.body().as_ref().unwrap();
                let form_info: FormInfo = serde_json::from_str(&body).expect("failed to deserialize form");
                Msg::GetFormSuccess(form_info)
            } else {
                Msg::GetFormFailure(format!("{}", response.body().as_ref().err().unwrap()))
            }
        })).expect("Failed to fetch schema");

        let state = State { 
            form: None,
            field_data: Vec::new(),
        };

        Form {
            link,
            state,
            task: Some(task),
            submit_tasks: Vec::new(),
            submit_completion: Vec::new(),
            message: html!{},
            props,
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
            Msg::GetFormFailure(error) => {
                self.message = html! {
                    <article class="message is-danger">
                        <div class="message-header">
                            <p>{"Failed to load form"}</p>
                        </div>
                        <div class="message-body">
                        {error}
                        </div>
                    </article>
                };
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

                let mappings = self.state.form.clone().unwrap().mappings.clone();
                for (i, mapping) in mappings.iter().enumerate() {
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
                    let task = FetchService::fetch(request, self.link.callback(move |response: Response<Result<String, anyhow::Error>>| {
                        if response.status().is_success() {
                            info!("Success!");
                            Msg::SubmitSuccess(i)
                        } else {
                            info!("Failure!");
                            Msg::SubmitFailure(format!("{}", response.body().as_ref().err().unwrap()))
                        }
                    })).expect("Failed to send entry request");

                    self.submit_tasks.push(task);
                    self.submit_completion.push(false);
                }
                true
            }
            Msg::SubmitSuccess(i) => {
                self.message = html! {
                    <article class="message is-primary">
                        <div class="message-header">
                            <p>{self.props.success_message_title.clone()}</p>
                            <button class="delete" aria-label="delete" onclick=self.link.callback(|_| Msg::ClearMessage)></button>
                        </div>
                        <div class="message-body">
                        {self.props.success_message_body.clone()}
                        </div>
                    </article>
                };
                // Mark this task as completed
                self.submit_completion[i] = true;
                // If all tasks are completed, clear the tasks
                if self.submit_completion.iter().all(|done| *done) {
                    self.submit_tasks.clear();
                    self.submit_completion.clear();
                }
                true
            }
            Msg::SubmitFailure(error) => {
                self.message = html! {
                    <article class="message is-danger">
                        <div class="message-header">
                            <p>{"Failed to submit form entry"}</p>
                            <button class="delete" aria-label="delete" onclick=self.link.callback(|_| Msg::ClearMessage)></button>
                        </div>
                        <div class="message-body">
                        {error}
                        </div>
                    </article>
                };
                self.submit_tasks.clear();
                self.submit_completion.clear();
                true
            }
            Msg::ClearMessage => {
                self.message = html!{};
                true
            }
            _ => true
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
                {self.message.clone()}
                <div>
                    {fields}
                </div>
                {submit}
            </div>
        }
    }
}
