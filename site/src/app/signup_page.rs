use yew::format::Json;
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Response, Request};

use yew_router::{agent::RouteRequest, prelude::*};

pub struct SignupPage {
    state: SignupData,
    link: ComponentLink<Self>,
    task: Option<FetchTask>,
    router: Box<dyn Bridge<RouteAgent>>,
    message: Html,
}

#[derive(Default, Serialize, Clone)]
struct SignupData {
    email: String,
    password: String,
}

pub enum Msg {
    SignupSuccess(String),
    SignupFailure(String),
    UpdateEmail(String),
    UpdatePassword(String),
    DoSignup,
    NoOp,
    ClearMessage,
}

impl Component for SignupPage {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let router = RouteAgent::bridge(link.callback(|_| Msg::NoOp));

        SignupPage { 
            state: Default::default(),
            link,
            task: None,
            router,
            message: html!{},
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
            Msg::SignupSuccess(data) => {
                info!("Signup success: {}", data);
                self.task = None;

                // Redirect to login page
                let route = Route::from("/login".to_string());
                self.router.send(RouteRequest::ChangeRoute(route));

                false
            }
            Msg::SignupFailure(error) => {
                self.task = None;

                self.message = html! {
                    <article class="message is-danger">
                        <div class="message-header">
                            <p>{"Failed to signup"}</p>
                            <button class="delete" aria-label="delete" onclick=self.link.callback(|_| Msg::ClearMessage)></button>
                        </div>
                        <div class="message-body">
                        {error}
                        </div>
                    </article>
                };

                true
            }
            Msg::DoSignup => {
                // TODO: Disable the login button to prevent duplicate reuqests
                let url = "/api/users";

                #[derive(Serialize)]
                struct RequestFormat {
                    user: SignupData,
                }
                let body = RequestFormat {
                    user: self.state.clone()
                };

                let request = Request::post(url)
                    .header("Content-Type", "application/json")
                    .body(Json(&body))
                    .expect("Failed to build request");

                self.task = Some(FetchService::fetch(request, self.link.callback(move |response: Response<Result<String, anyhow::Error>>| {
                    debug!("Response received from {}", url);
                    let (meta, result) = response.into_parts();
                    if meta.status.is_success() {
                        Msg::SignupSuccess(result.unwrap())
                    } else {
                        Msg::SignupFailure(result.unwrap())
                    }
                })).expect("Failed to get fetch task"));


                true
            }
            Msg::ClearMessage => {
                self.message = html!{};
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
                {self.message.clone()}
                <label class="label" for="email">{"Email"}</label>
                <input type="text"
                       value=self.state.email 
                       name="email" 
                       oninput=self.link.callback(|e: InputData| Msg::UpdateEmail(e.value))
                       class="input"
                       />
                <label class="label" for="password">{"Password"}</label>
                <input type="password"
                       value=self.state.password 
                       name="password" 
                       oninput=self.link.callback(|e: InputData| Msg::UpdatePassword(e.value))
                       class="input"
                       />
                <button 
                    onclick=self.link.callback(|_| Msg::DoSignup) 
                    class="button"
                    >{"Sign up"}</button>
            </>
        }
    }
}
