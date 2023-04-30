use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};

use crate::api;
use crate::error::Error;
use crate::types::EntryInfo;

pub mod entry_item;
use entry_item::EntryItem;

pub struct ViewEntries {
    state: State,
    fetch: api::Entry,
    link: ComponentLink<Self>,
    task: Option<FetchTask>,
    props: Props,
}

#[derive(Default)]
struct State {
    entries: Vec<EntryInfo>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub schema_id: u32,
}

pub enum Msg {
    FetchSuccess(Vec<EntryInfo>),
    FetchFailure(Error),
}

impl Component for ViewEntries {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        // Attempt to fetch entries
        let mut fetch = api::Entry::new();
        let task = fetch.get_by_schema_id(
            props.schema_id,
            link.callback(|response: Result<Vec<EntryInfo>, Error>| match response {
                Ok(list) => Msg::FetchSuccess(list),
                Err(err) => Msg::FetchFailure(err),
            }),
        );

        ViewEntries {
            state: Default::default(),
            fetch,
            link,
            task: Some(task),
            props,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::FetchSuccess(data) => {
                self.state.entries = data;
                self.task = None;
                true
            }
            Msg::FetchFailure(_error) => {
                self.task = None;
                // TODO: Respond to this
                true
            }
            _ => false,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        html! {
            <>
                <h1 class="title">{format!("Entries for schema id {}", self.props.schema_id)}</h1>
                <div class="container">
                    {self.state.entries.iter().map(|entry| {
                       html! { <EntryItem entry={entry.clone()} /> }
                    }).collect::<Html>()}
                </div>
            </>
        }
    }
}
