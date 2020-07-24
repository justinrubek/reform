use yew::prelude::*;

use crate::auth_agent::{is_authenticated, set_token};

mod signup_page;
use signup_page::SignupPage;

mod login_page;
use login_page::LoginPage;

mod schema_page;
use schema_page::SchemaPage;

mod form_page;
use form_page::FormPage;

pub struct App {
    link: ComponentLink<Self>,
    state: State,
}

struct State {
}

pub enum Msg {
    LogIn(String),
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

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::LogIn(data) => {
                #[derive(Deserialize)]
                struct LoginResponseData {
                    email: String,
                    token: String
                }
                #[derive(Deserialize)]
                struct LoginResponse {
                    user: LoginResponseData
                }

                let response: LoginResponse = serde_json::from_str(&data).expect("Login token format invalid");
                set_token(Some(response.user.token));
                true
            }

            _ => true
        }
    }

    fn view(&self) -> Html {
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
                            <SchemaPage />
                            <FormPage />
                        </div>
                    </section>
                }
            }
        }
    }
}
