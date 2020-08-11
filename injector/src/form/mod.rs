use failure::Error;

use yew::format::Nothing;
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};

mod field;
use field::{Field, FormField};


#[derive(Clone, PartialEq, Deserialize)]
struct FormInfo {
    id: u32,
    name: String,
    fields: Vec<FormField>,
    mappings: Vec<serde_json::Value>,
}

pub struct Form {
    link: ComponentLink<Self>,
    state: State,
    task: Option<FetchTask>,
    fetch: FetchService,
}

struct FieldData {
    name: String,
    data: String,
}

struct State {
    form: Option<FormInfo>,
    field_data: Vec<FieldData>,
}

pub enum Msg {
    GetFormSuccess(FormInfo),
    GetFormFailure,
    UpdateField(usize, String),
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
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        // TODO: Disable/enable button as needed
        match msg {
            Msg::GetFormSuccess(form_info) => {
                // Create empty data for each field
                let mut field_data = Vec::new();
                form_info.fields.iter().for_each(|field| {
                    field_data.push(FieldData {
                        name: field.name.clone(),
                        data: String::new(),
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
                true
            }
            Msg::SubmitSuccess => {
                true
            }
            Msg::SubmitFailure => {
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
        html! {
            <div>
                <p>{"Hello form"}</p>
                {fields}
            </div>
        }
    }
}
