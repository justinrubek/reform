use anyhow::Error;

use yew::format::Json;
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Response, Request};

pub struct LoginPage {
    state: LoginData,
    fetch: FetchService,
    onlogin: Callback<String>,
    link: ComponentLink<Self>,
    task: Option<FetchTask>,
}

#[derive(Default, Serialize, Clone)]
struct LoginData {
    email: String,
    password: String,
}

pub enum Msg {
    LoginSuccess(String),
    LoginFailure,
    UpdateEmail(String),
    UpdatePassword(String),
    DoLogin,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub onlogin: Callback<String>,
}

impl Component for LoginPage {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        LoginPage { 
            state: Default::default(),
            fetch: FetchService::new(),
            onlogin: props.onlogin,
            link,
            task: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateEmail(email) => {
                self.state.email = email;
                true
            }
            Msg::UpdatePassword(password) => {
                self.state.password = password;
                true
            }
            Msg::LoginSuccess(token) => {
                info!("Login success");
                self.task = None;
                self.onlogin.emit(token); 
                false
            }
            Msg::LoginFailure => {
                warn!("Login failure");
                self.task = None;
                // TODO: Display to user
                true
            }
            Msg::DoLogin => {
                // TODO: Disable the login button to prevent duplicate reuqests
                debug!("DoLogin");
                let url = "/api/login";


                let request = Request::post(url)
                    .header("Content-Type", "application/json")
                    .body(Json(&self.state))
                    .expect("Failed to build request");

                self.task = Some(self.fetch.fetch(request, self.link.callback(move |response: Response<Result<String, anyhow::Error>>| {
                    debug!("Response received from {}", url);
                    let (meta, result) = response.into_parts();
                    if meta.status.is_success() {
                        Msg::LoginSuccess(result.unwrap())
                    } else {
                        Msg::LoginFailure
                    }
                })).expect("Failed to get fetch task"));


                true
            }
            _ => false
        }
    }

    fn view(&self) -> Html {
        html! {
            <>
                <input type="text"
                       value=self.state.email 
                       name="email" 
                       oninput=self.link.callback(|e: InputData| Msg::UpdateEmail(e.value))
                       />
                <input type="password"
                       value=self.state.password 
                       name="password" 
                       oninput=self.link.callback(|e: InputData| Msg::UpdatePassword(e.value))
                       />
                <button onclick=self.link.callback(|_| Msg::DoLogin) >{"Log in"}</button>
            </>
        }
    }
}
