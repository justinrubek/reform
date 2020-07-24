use yew::format::Json;
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Response, Request};

use crate::auth_agent;
use crate::error::Error;
use crate::types::EntryInfo;


/* EntryPage:
 * Actions to faciliate:
 *  Creation of schema
 *  Deletion of schema -> May be unwise to do this, as it is in use?
 *  Edit by modify -> This may not be needed, as they can just create a new one themselves
 *  Get identifier for schema for use elsewhere
 */

pub struct EntryItem {
    state: EntryItemState,
    link: ComponentLink<Self>,
}

struct EntryItemState {
    entry: EntryInfo,
}

pub enum Msg {
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub entry: Option<EntryInfo>,
}


impl Component for EntryItem {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        // Attempt to fetch schemas
        let state = EntryItemState {
            entry: props.entry.expect("EntryInfo with no entry"),
        };

        EntryItem { 
            state,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        true
    }
    fn view(&self) -> Html {
        html! {
            <>
                <div class="media-left">
                    <p>{format!("id: {}", self.state.entry.id)}</p>
                </div>
                <div class="media-center">
                    <p>{format!("data: {}", self.state.entry.data)}</p>
                </div>
                <div class="media-right">
                    <p>{format!("s_id: {}", self.state.entry.schema_id)}</p>
                </div>
            </>
        }
    }
}
