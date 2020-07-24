use yew::components::Select;
use yew::format::Json;
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Response, Request};

#[derive(Clone, Default, PartialEq)]
pub struct Mapping {
    dummy: String,
}

#[derive(Default)]
struct State {
    mapping: Mapping,
}

pub enum Msg {
    OnChange,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[props(required)]
    pub onchange: Callback<Mapping>,
    #[props(required)]
    pub mapping: Mapping,
}

pub struct MappingItem {
    state: State,
    link: ComponentLink<Self>,
    onchange: Callback<Mapping>,
}

impl Component for MappingItem {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let state = State {
            mapping: props.mapping,
        };

        MappingItem { 
            state: state,
            link,
            onchange: props.onchange,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::OnChange => {
                self.onchange.emit(self.state.mapping.clone());
                true
            }
            _ => false
        }
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <p>{"Mapping"}</p>
            </div>
        }
    }
}

