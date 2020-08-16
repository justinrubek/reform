use yew::prelude::*;
use yew_router::{route::Route, router::Router, service::RouteService, prelude::*};

use crate::auth_agent::{is_authenticated, set_token};
use crate::app::form::view::ViewForms;
use crate::app::schema::view::ViewSchemas;

use super::AppRoute;

pub struct DashboardComponent {
    link: ComponentLink<Self>,
    state: State,
}

struct State {
}

pub enum Msg {
}

impl Component for DashboardComponent {
    type Message = Msg;
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
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

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <h1 class="title">{"Dashboard"}</h1>
                <ViewSchemas />
                <RouterButton<AppRoute> route=AppRoute::CreateSchema classes="button">{"New Schema"}</RouterButton<AppRoute>>
                <ViewForms />
                <RouterButton<AppRoute> route=AppRoute::CreateForm classes="button">{"New Form"}</RouterButton<AppRoute>>
            </div>
        }
    }
}
