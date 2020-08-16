use yew::format::Json;
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Response, Request};

use crate::auth_agent;
use crate::error::Error;
use crate::types::FormInfo;

pub struct FormItem {
    state: State,
    link: ComponentLink<Self>,
    props: Props,
}

struct State {
}

pub enum Msg {
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub form: FormInfo,
}


impl Component for FormItem {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let state = State {
        };

        FormItem { 
            state,
            link,
            props,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            _ => true,
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
        html! {
            <div class="media">
                <div class="media-left">
                    <p>{format!("id: {}", self.props.form.id)}</p>
                </div>
                <div class="media-left">
                    <p>{format!("name: {}", self.props.form.name)}</p>
                </div>
                <div class="media-right">
                    <button class="button" >{"edit"}</button>
                </div>
            </div>
        }
    }
}
