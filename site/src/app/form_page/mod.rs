use yew::format::Json;
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Response, Request};

use crate::auth_agent;
use crate::error::Error;
use crate::types::{FormInfo, FormCreateInfo};

mod field;
use field::{Field, FieldItem};

pub mod view;

mod mapping_page;
use mapping_page::MappingPage;

pub struct FormPage {
    state: State,
    link: ComponentLink<Self>,
    onback: Option<Callback<()>>,
    api_handler: auth_agent::Form,
    task: Option<FetchTask>,
}

#[derive(Default)]
struct State {
    fields: Vec<Field>,
}

pub enum Msg {
    UpdateField(usize, Field),
    AddField,
    PostForm,
    CreateFormSuccess(FormInfo),
    CreateFormFailure,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub onback: Option<Callback<()>>,
}


impl Component for FormPage {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        FormPage { 
            state: Default::default(),
            link,
            onback: props.onback,
            api_handler: auth_agent::Form::new(),
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
            Msg::CreateFormSuccess(form_info) => {
                true
            }
            Msg::CreateFormFailure => {
                true
            }
            Msg::PostForm => {
                // TODO: Disable the login button to prevent duplicate reuqests

                // TODO: Implement name choice
                let name = "Form";

                let fields = json!(self.state.fields);

                let mappings = json!([]);

                let form_info = FormCreateInfo {
                    name: name.into(),
                    fields: fields,
                    mappings: mappings,
                };

                self.task = Some(self.api_handler.create(form_info, self.link.callback(move |response: Result<FormInfo, Error>| {
                    debug!("Response received for CreateSchema");
                    if response.is_ok() {
                        Msg::CreateFormSuccess(response.unwrap())
                    } else {
                        warn!("{:?}", response.err());
                        Msg::CreateFormFailure
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
                <FieldItem field=field onchange=&self.link.callback(move |field| Msg::UpdateField(i, field))/>
            }
        }).collect::<Html>();

        html! {
            <>
                <h1 class="title">{"Forms"}</h1>
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
                                <th>{"label"}</th>
                            </tr>
                        </thead>
                        {fields}
                    </table>
                    <MappingPage />
                    <button class="button " onclick=self.link.callback(|_| Msg::PostForm)>{"create form"}</button>
                </div>
            </>
        }
    }
}

