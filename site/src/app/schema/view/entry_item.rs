
use yew::prelude::*;




use crate::types::EntryInfo;


/* EntryPage:
 * Actions to faciliate:
 *  Creation of schema
 *  Deletion of schema -> May be unwise to do this, as it is in use?
 *  Edit by modify -> This may not be needed, as they can just create a new one themselves
 *  Get identifier for schema for use elsewhere
 */

pub struct EntryItem {
    link: ComponentLink<Self>,
    props: Props,
}

pub enum Msg {
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub entry: EntryInfo,
}


impl Component for EntryItem {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        EntryItem { 
            link,
            props,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
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
                <div class="media-left">
                    <p>{format!("id: {}", self.props.entry.id)}</p>
                </div>
                <div class="media-center">
                    <p>{format!("data: {}", self.props.entry.data)}</p>
                </div>
                <div class="media-right">
                    <p>{format!("s_id: {}", self.props.entry.schema_id)}</p>
                </div>
            </>
        }
    }
}
