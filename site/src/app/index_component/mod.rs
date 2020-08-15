use yew::prelude::*;
use yew_router::{route::Route, service::RouteService};

use crate::auth_agent::{is_authenticated, set_token};

use super::AppRoute;

pub struct IndexComponent {
    link: ComponentLink<Self>,
    state: State,
}

struct State {
}

pub enum Msg {
    ChangeRoute(AppRoute),
}

impl Component for IndexComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let state = State { 
        };

        IndexComponent {
            link,
            state,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <p>{"Index"}</p>
        }
    }
}
