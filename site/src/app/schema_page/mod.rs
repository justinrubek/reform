use yew::format::Json;
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Response, Request};

use crate::auth_agent;
use crate::error::Error;
use crate::types::SchemaInfo;

mod schema_item;
use schema_item::SchemaItem;

pub mod entry_item;
pub mod create_schema;

/* SchemaPage:
 * Actions to faciliate:
 *  Creation of schema
 *  Deletion of schema -> May be unwise to do this, as it is in use?
 *  Edit by modify -> This may not be needed, as they can just create a new one themselves
 *  Get identifier for schema for use elsewhere
 */

pub struct SchemaPage {
    state: SchemaState,
    fetch: auth_agent::Schema,
    link: ComponentLink<Self>,
    task: Option<FetchTask>,
}

#[derive(Default)]
struct SchemaState {
    schemas: Vec<SchemaInfo>,
}

pub enum Msg {
    FetchSuccess(Vec<SchemaInfo>),
    FetchFailure(Error),
}

impl Component for SchemaPage {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        // Attempt to fetch schemas
        let mut fetch = auth_agent::Schema::new();
        let task = fetch.get_all(link.callback(|response: Result<Vec<SchemaInfo>, Error>| {
            match response {
                Ok(list) => Msg::FetchSuccess(list),
                Err(err) => Msg::FetchFailure(err),
            }
        }));

        SchemaPage { 
            state: Default::default(),
            fetch: fetch,
            link,
            task: Some(task),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        debug!("Message - Schema");
        match msg {
            Msg::FetchSuccess(data) => {
                debug!("Success - Schema");
                self.state.schemas = data;
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

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <h1 class="title">{"Schemas"}</h1>
                <div class="container">
                    {self.state.schemas.iter().map(|schema| {
                       html! { <SchemaItem schema={schema.clone()} /> }
                    }).collect::<Html>()}
                </div>
            </>
        }
    }
}

