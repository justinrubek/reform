use yew::prelude::*;
use yew_router::{route::Route, service::RouteService};

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
    props: Props,
}

struct State {
}

pub enum Msg {
    ChangeRoute(AppRoute),
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub route: DashboardRoute,
}

impl Component for DashboardComponent {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let state = State { 
        };

        DashboardComponent {
            link,
            state,
            props,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
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
            <p>{"Dashboard"}</p>
        }
    }
}
