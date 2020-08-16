use yew::format::Json;
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Response, Request};

use crate::auth_agent;
use crate::error::Error;
use crate::types::{FormInfo, FormCreateInfo};

mod field;
use field::{Field, FieldItem};

mod mapping_page;
use mapping_page::MappingPage;
use mapping_page::Mapping;

pub struct CreateForm {
    state: State,
    link: ComponentLink<Self>,
    api_handler: auth_agent::Form,
    task: Option<FetchTask>,
}

#[derive(Default)]
struct State {
    fields: Vec<Field>,
    mappings: Vec<Mapping>,
    name: String,
}

pub enum Msg {
    UpdateName(String),
    UpdateField(usize, Field),
    AddField,
    PostForm,
    CreateFormSuccess(FormInfo),
    CreateFormFailure,
    UpdateMappings(Vec<Mapping>),
}

impl Component for CreateForm {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        CreateForm { 
            state: Default::default(),
            link,
            api_handler: auth_agent::Form::new(),
            task: None,
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
            Msg::CreateFormSuccess(form_info) => {
                true
            }
            Msg::CreateFormFailure => {
                // TODO: Respond
                true
            }
            Msg::PostForm => {
                // TODO: Disable the login button to prevent duplicate reuqests

                // TODO: Implement name choice
                let fields = json!(self.state.fields);

                let mappings = json!(self.state.mappings);

                let form_info = FormCreateInfo {
                    name: self.state.name.clone(),
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
            Msg::UpdateMappings(mappings) => {
                self.state.mappings = mappings;
                true
            }
            _ => false
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let fields = self.state.fields.iter().enumerate().map(|(i, field)| {
            html! {
                <FieldItem field=field onchange=&self.link.callback(move |field| Msg::UpdateField(i, field))/>
            }
        }).collect::<Html>();

        html! {
            <>
                <h1 class="title">{"Form creator"}</h1>
                <div class="container">
                    <div class="media">
                        <h2 class="title media-left">{"Fields"}</h2> 
                        <label class="label" for="form_name">{"Name"}</label>
                        <input class="input" type="text" name="form_name" value=self.state.name
                        oninput=self.link.callback(|e: InputData| Msg::UpdateName(e.value))
                        />
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
                    <MappingPage onchange=self.link.callback(|mappings| Msg::UpdateMappings(mappings)) />
                    <button class="button " onclick=self.link.callback(|_| Msg::PostForm)>{"create form"}</button>
                </div>
            </>
        }
    }
}

