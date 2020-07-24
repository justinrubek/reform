use yew::format::Json;
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Response, Request};

use crate::auth_agent;
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
    state: SchemaItemState,
    link: ComponentLink<Self>,
    fetch: auth_agent::Entry,
    task: Option<FetchTask>,
}

struct SchemaItemState {
    schema: SchemaInfo,
    entries: Vec<EntryInfo>,
}

pub enum Msg {
    FetchSuccess(Vec<EntryInfo>),
    FetchFailure(Error),
    SendFetch,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub schema: Option<SchemaInfo>,
}


impl Component for SchemaItem {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        // Attempt to fetch schemas
        let state = SchemaItemState {
            schema: props.schema.expect("SchemaItem rendered with no schema"),
            entries: Default::default(),
        };

        SchemaItem { 
            state,
            link,
            fetch: auth_agent::Entry::new(),
            task: None,
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
                let task = self.fetch.get_by_schema_id(self.state.schema.id,
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
// { <EntryItem entry={Some(entry.clone())} /> }
    fn view(&self) -> Html {
        let entries = self.state.entries.iter().map(|entry| {
           html! { 
               <div class="media">
                   <EntryItem entry={Some(entry.clone())} />
               </div>
           }
        }).collect::<Html>();
        html! {
            <div class="media">
                <div class="media-left">
                    <p>{format!("id: {}", self.state.schema.id)}</p>
                </div>
                <div class="media-content">
                    <p>{format!("data: {}", self.state.schema.data)}</p>
                    {entries}
                </div>
                <div class="media-right">
                    <button class="button" onclick=self.link.callback(|_| Msg::SendFetch)>{"Get entries"}</button>
                </div>
            </div>
        }
    }
}
