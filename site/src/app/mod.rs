use yew::prelude::*;

mod signup_page;
use signup_page::SignupPage;

mod login_page;
use login_page::LoginPage;

pub struct App {
    link: ComponentLink<Self>,
    state: State,
}

struct State {
    logged_in: bool,
}

pub enum Msg {
    LogIn(String),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let state = State { 
            logged_in: false 
        };

        App {
            link,
            state,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::LogIn(_token) => {
                self.state.logged_in = true;             
                true
            }

            _ => true
        }
    }

    fn view(&self) -> Html {
        match self.state.logged_in {
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
                    <p>{"You're logged in "}</p>
                }
            }
        }
    }
}
