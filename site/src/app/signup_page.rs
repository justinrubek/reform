use yew::format::Json;
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Response, Request};

pub struct SignupPage {
    state: SignupData,
    fetch: FetchService,
    link: ComponentLink<Self>,
    task: Option<FetchTask>,
}

#[derive(Default, Serialize, Clone)]
struct SignupData {
    email: String,
    password: String,
}

pub enum Msg {
    SignupSuccess(String),
    SignupFailure,
    UpdateEmail(String),
    UpdatePassword(String),
    DoSignup,
}

impl Component for SignupPage {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        SignupPage { 
            state: Default::default(),
            fetch: FetchService::new(),
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
            Msg::SignupSuccess(token) => {
                info!("Signup success: {}", token);
                self.task = None;
                false
            }
            Msg::SignupFailure => {
                warn!("Signup failure");
                self.task = None;
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

                self.task = Some(self.fetch.fetch(request, self.link.callback(move |response: Response<Result<String, failure::Error>>| {
                    debug!("Response received from {}", url);
                    let (meta, result) = response.into_parts();
                    if meta.status.is_success() {
                        Msg::SignupSuccess(result.unwrap())
                    } else {
                        Msg::SignupFailure
                    }
                })));


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
                <button onclick=self.link.callback(|_| Msg::DoSignup) >{"Sign up"}</button>
            </>
        }
    }
}
