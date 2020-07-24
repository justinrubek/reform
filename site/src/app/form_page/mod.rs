use yew::format::Json;
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Response, Request};

use crate::auth_agent;
use crate::error::Error;

mod field;
use field::{Field, FieldItem};

mod mapping_page;
use mapping_page::MappingPage;

pub struct FormPage {
    state: State,
    link: ComponentLink<Self>,
    onback: Option<Callback<()>>,
}

#[derive(Default)]
struct State {
    fields: Vec<Field>,
}

pub enum Msg {
    UpdateField(usize, Field),
    AddField,
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
                </div>
            </>
        }
    }
}

