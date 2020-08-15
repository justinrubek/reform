use yew::format::Json;
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Response, Request};

use crate::auth_agent;
use crate::error::Error;
use crate::types::{FormInfo, FormList};

mod form_item;
use form_item::FormItem;

pub struct ViewForms {
    state: State,
    fetch: auth_agent::Form,
    link: ComponentLink<Self>,
    task: Option<FetchTask>,
}

#[derive(Default)]
struct State {
    forms: Vec<FormInfo>,
}

pub enum Msg {
    FetchSuccess(Vec<FormInfo>),
    FetchFailure(Error),
}


impl Component for ViewForms {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        // Attempt to fetch forms
        let mut fetch = auth_agent::Form::new();
        let task = fetch.get_all(link.callback(|response: Result<Vec<FormInfo>, Error>| {
            match response {
                Ok(list) => Msg::FetchSuccess(list),
                Err(err) => Msg::FetchFailure(err),
            }
        }));

        ViewForms { 
            state: Default::default(),
            fetch: fetch,
            link,
            task: Some(task),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        debug!("Message - ViewForms");
        match msg {
            Msg::FetchSuccess(data) => {
                debug!("Success - ViewForms");
                self.state.forms = data;
                self.task = None;
                true
            }
            Msg::FetchFailure(error) => {
                self.task = None;
                // TODO: Respond to this
                error!("error: {}", error);
                true
            }
            _ => false
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <h1 class="title">{"Forms"}</h1>
                <div class="container">
                    {self.state.forms.iter().map(|form| {
                       html! { <FormItem form={form.clone()} /> }
                    }).collect::<Html>()}
                </div>
            </>
        }
    }
}

