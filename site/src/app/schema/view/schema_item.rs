
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Response, Request};

use yew_router::{prelude::*};

use crate::app::AppRoute;
use crate::api;
use crate::error::Error;
use crate::types::{EntryInfo, SchemaInfo};

use super::entry_item::EntryItem;

/* SchemaPage:
 * Actions to faciliate:
 *  Creation of schema
 *  Deletion of schema -> May be unwise to do this, as it is in use?
 *  Edit by modify -> This may not be needed, as they can just create a new one themselves
 *  Get identifier for schema for use elsewhere
 */

pub struct SchemaItem {
    state: State,
    link: ComponentLink<Self>,
    fetch: api::Entry,
    task: Option<FetchTask>,
    props: Props,
}

struct State {
    entries: Vec<EntryInfo>,
}

pub enum Msg {
    FetchSuccess(Vec<EntryInfo>),
    FetchFailure(Error),
    SendFetch,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub schema: SchemaInfo,
}


impl Component for SchemaItem {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        // Attempt to fetch schemas
        let state = State {
            entries: Default::default(),
        };

        SchemaItem { 
            state,
            link,
            fetch: api::Entry::new(),
            task: None,
            props,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::FetchSuccess(list) => {
                self.task = None;
                self.state.entries = list;
                true
            }
            Msg::FetchFailure(error) => {
                self.task = None;
                error!("{}", error);
                true
            }
            Msg::SendFetch => {
                let task = self.fetch.get_by_schema_id(self.props.schema.id,
                    self.link.callback(|response: Result<Vec<EntryInfo>, Error>| {
                        match response {
                            Ok(list) => Msg::FetchSuccess(list),
                            Err(err) => Msg::FetchFailure(err),
                        }
                }));
                self.task = Some(task);
                false
            }
            _ => false,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            self.link.send_message(Msg::SendFetch);
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let entries = self.state.entries.iter().map(|entry| {
           html! { 
               <div class="media">
                   <EntryItem entry={entry.clone()} />
               </div>
           }
        }).collect::<Html>();
        html! {
            <div class="media">
                <div class="media-left">
                    <p>{format!("name: {}", self.props.schema.name)}</p>
                </div>
                <div class="media-content">
                    <p>{format!("id: {}", self.props.schema.id)}</p>
                    <p>{format!("data: {}", self.props.schema.data)}</p>
                    {entries}
                </div>
                <div class="media-right">
                    <RouterButton<AppRoute> route=AppRoute::ViewEntries(self.props.schema.id) classes="button">{"view entries"}</RouterButton<AppRoute>>
                </div>
            </div>
        }
    }
}
