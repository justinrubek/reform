use yew::prelude::*;
use yew_router::{route::Route, RouteService};

use crate::auth_agent::{is_authenticated, set_token};
use super::AppRoute;

#[derive(Clone, Debug, PartialEq, Switch)]
pub enum DashboardRoute {
    #[to = "/form"]
    CreateForm,
    #[to = "/schema"]
    CreateSchema,
}

pub struct DashboardComponent {
    link: ComponentLink<Self>,
    state: State,
}

struct State {
}

pub enum Msg {
    ChangeRoute(AppRoute),
}

#[derive(Clone, Properties)]
pub struct Props {
    pub route: DashboardRoute,
}

impl Component for DashboardComponent {
    type Message = Msg;
    type Properties = Props;

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let state = State { 
        };

        DashboardComponent {
            link,
            state,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <p>{"Dashboard"}</p>
        }
    }
}
