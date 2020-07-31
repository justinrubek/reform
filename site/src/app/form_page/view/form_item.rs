use yew::format::Json;
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Response, Request};

use crate::auth_agent;
use crate::error::Error;
use crate::types::FormInfo;

pub struct FormItem {
    state: State,
    link: ComponentLink<Self>,
}

struct State {
    form: FormInfo,
}

pub enum Msg {
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub form: Option<FormInfo>,
}


impl Component for FormItem {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let state = State {
            form: props.form.expect("FormItem rendered with no form"),
        };

        FormItem { 
            state,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            _ => true,
        }
    }

    fn view(&self) -> Html {
        html! {
            <div class="media">
                <div class="media-left">
                    <p>{format!("id: {}", self.state.form.id)}</p>
                </div>
                <div class="media-left">
                    <p>{format!("name: {}", self.state.form.name)}</p>
                </div>
                <div class="media-right">
                    <button class="button" >{"edit"}</button>
                </div>
            </div>
        }
    }
}
