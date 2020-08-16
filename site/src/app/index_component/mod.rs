use yew::prelude::*;
use yew_router::{route::Route, service::RouteService, prelude::*};

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

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <h2>{"Index"}</h2>
                <RouterButton<AppRoute> route=AppRoute::Login classes="button">{"Log in"}</RouterButton<AppRoute>>
                <RouterButton<AppRoute> route=AppRoute::Signup classes="button">{"Create an account"}</RouterButton<AppRoute>>
            </>

        }
    }
}
