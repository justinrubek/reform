use yew::prelude::*;
use yew_router::{prelude::*};



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

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <h1 class="title">{"Welcome to reform"}</h1>
                <article class="media">
                    <div class="media-content">
                        <div class="content">
                        <RouterButton<AppRoute> route=AppRoute::Login classes="button">{"Log in"}</RouterButton<AppRoute>>
                        <RouterButton<AppRoute> route=AppRoute::Signup classes="button">{"Create an account"}</RouterButton<AppRoute>>
                        </div>
                    </div>
                </article>
            </>

        }
    }
}
