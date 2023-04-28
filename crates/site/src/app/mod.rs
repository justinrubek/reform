use yew::prelude::*;
use yew_router::prelude::*;

use yew_router::router::Router;

mod index_component;
use index_component::IndexComponent;

mod dashboard_component;
use dashboard_component::DashboardComponent;

mod signup_page;
use signup_page::SignupPage;

mod login_page;
use login_page::LoginPage;

mod schema;

use schema::create::CreateSchema;

mod form;
use form::create::CreateForm;


mod entry;
use entry::view::ViewEntries;

#[derive(Clone, Debug, Switch)]
pub enum AppRoute {
    #[to = "/login"]
    Login,
    #[to = "/signup"]
    Signup,
    #[to = "/dashboard/view/schema/{schema_id}/entries"]
    ViewEntries(u32),
    #[to = "/dashboard/create/form"]
    CreateForm,
    #[to = "/dashboard/create/schema"]
    CreateSchema,
    #[to = "/dashboard"]
    Dashboard,
    #[to = "/!"]
    Index,
}

pub struct App {
    link: ComponentLink<Self>,
    state: State,
}

struct State {
}

pub enum Msg {
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let state = State { 
        };

        App {
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
            <section class="section">
            <div class="container">
                <Router<AppRoute, ()>
                    render = Router::render(|switch: AppRoute| {
                        match switch {
                            AppRoute::Index => html!{<IndexComponent />},
                            AppRoute::Login => html!{<LoginPage />},
                            AppRoute::Signup => html!{<SignupPage />},
                            AppRoute::CreateForm => html!{<CreateForm />},
                            AppRoute::CreateSchema => html!{<CreateSchema />},
                            AppRoute::Dashboard => html!{<DashboardComponent />},
                            AppRoute::ViewEntries(schema_id) => html!{<ViewEntries schema_id=schema_id />},
                        }
                    })
                />
            </div>
            </section>
        }
    }
}
/*
        match is_authenticated() {
            false => html! {
                <div>
                    <h1>{"Signup"}</h1>
                    <SignupPage />

                    <h1>{"Login"}</h1>
                    <LoginPage onlogin=&self.link.callback(|tok| Msg::LogIn(tok)) />

                </div>
            },

            true => {
                html! {
                    <section class="section">
                        <div class="container">
                            <p>{"You're logged in!"}</p>
                            <h1 class="title">{"Schema"}</h1>
                            <h2>{"View schema"}</h2>
                            <SchemaPage />
                            <CreateSchema />
                            <ViewForms />
                            <FormPage />
                        </div>
                    </section>
                }
            }
*/
