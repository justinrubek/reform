use anyhow::Error;

use yew::format::Json;
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Response, Request};

use yew_router::{agent::RouteRequest, prelude::*};

use crate::auth_agent::set_token;

pub struct LoginPage {
    state: LoginData,
    link: ComponentLink<Self>,
    task: Option<FetchTask>,
    router: Box<dyn Bridge<RouteAgent>>,
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
    NoOp,
}

impl Component for LoginPage {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let router = RouteAgent::bridge(link.callback(|_| Msg::NoOp));

        LoginPage { 
            state: Default::default(),
            link,
            task: None,
            router,
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
            Msg::LoginSuccess(data) => {
                info!("Login success");
                self.task = None;

                // Extract data from response
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

                // Store login token 
                set_token(Some(response.user.token));

                // Navigate to dashboard
                let route = Route::from("/dashboard".to_string());
                self.router.send(RouteRequest::ChangeRoute(route));

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

                self.task = Some(FetchService::fetch(request, self.link.callback(move |response: Response<Result<String, anyhow::Error>>| {
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

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        false
    }


    fn view(&self) -> Html {
        html! {
            <>
                <label class="label" for="email">{"Email"}</label>
                <input type="text"
                       value=self.state.email 
                       name="email" 
                       oninput=self.link.callback(|e: InputData| Msg::UpdateEmail(e.value))
                       class="input"
                       />
                <label class="label" for="password">{"password"}</label>
                <input type="password"
                       value=self.state.password 
                       name="password" 
                       oninput=self.link.callback(|e: InputData| Msg::UpdatePassword(e.value))
                       class="input"
                       />
                <button 
                    onclick=self.link.callback(|_| Msg::DoLogin) 
                    class="button"
                >{"Log in"}</button>
            </>
        }
    }
}
