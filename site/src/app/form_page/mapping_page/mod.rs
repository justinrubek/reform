use yew::format::Json;
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Response, Request};

use crate::auth_agent;
use crate::error::Error;

mod mapping;
pub use mapping::{Mapping, MappingItem};

pub struct MappingPage {
    state: State,
    link: ComponentLink<Self>,
    onchange: Callback<Vec<Mapping>>,
}

#[derive(Default)]
struct State {
    mappings: Vec<Mapping>,
}

pub enum Msg {
    OnChange,
    UpdateMapping(usize, Mapping),
    AddMapping,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[props(required)]
    pub onchange: Callback<Vec<Mapping>>,
}


impl Component for MappingPage {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        MappingPage { 
            state: Default::default(),
            link,
            onchange: props.onchange,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::OnChange => {
                self.onchange.emit(self.state.mappings.clone());
                true
            }
            Msg::UpdateMapping(i, mapping) => {
                self.state.mappings[i] = mapping;
                self.link.send_message(Msg::OnChange);
                true
            }
            Msg::AddMapping => {
                self.state.mappings.push(Default::default()); 
                self.link.send_message(Msg::OnChange);
                true
            }
            _ => false
        }
    }

    fn view(&self) -> Html {
        let mappings = self.state.mappings.iter().enumerate().map(|(i, mapping)| {
            html! {
                <MappingItem mapping=mapping onchange=&self.link.callback(move |mapping| {
                    Msg::UpdateMapping(i, mapping)
                }) />
            }
        }).collect::<Html>();

        html! {
            <div class="column">
                <div class="media">
                    <h2 class="title media-left">{"Mappings"}</h2> 
                    <button class="button media-right" onclick=self.link.callback(|_| Msg::AddMapping)>{"add mapping"}</button>
                </div>
                {mappings}
            </div>
        }
    }
}

