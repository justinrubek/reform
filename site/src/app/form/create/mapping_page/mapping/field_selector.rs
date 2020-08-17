

use stdweb::traits::{IEvent, IDragEvent};


use yew::prelude::*;



// Allow for text->string and choice->string
fn check_compatible_types(from: &str, to: &str) -> bool {
    // If they're the same, then they're compatible
    if from == to {
        return true;
    }

    if from == "text" && to == "string" || from == "string" && to == "text" {
        return true;
    }

    if from == "choice" && to == "string" {
        return true;
    }

    false
}

#[derive(Default)]
struct State {
    name: String,
    ftype: String,
    selected_field: Option<String>
}

pub enum Msg {
    OnChange,
    OnDrop(DragDropEvent),
    OnDragover(DragOverEvent),
    Nop,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub onchange: Callback<(String, String)>,
    pub name: String,
    pub ftype: String,
}

pub struct FieldSelector {
    state: State,
    link: ComponentLink<Self>,
    props: Props,
}

impl Component for FieldSelector {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let state = State {
            name: props.name.clone(),
            ftype: props.ftype.clone(),
            selected_field: None,
        };

        FieldSelector { 
            state,
            link,
            props
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::OnChange => {
                if let Some(selected) = &self.state.selected_field {
                    self.props.onchange.emit((selected.clone(), self.state.name.clone()));
                } else {
                    // FIXME: We currently don't support clearing the contents of a field
                    panic!("Attempted to update mapping without selecting an item");
                }
                
                true
            }
            Msg::OnDrop(event) => {
                debug!("Dropped!");
                event.prevent_default();

                let data_transfer = event.data_transfer().expect("No data transfer");

                // Verify type contraints
                let ftype = data_transfer.get_data("field-type");
                debug!("types ({}) ({})", ftype, self.state.ftype);
                if check_compatible_types(&ftype, &self.state.ftype) {
                    // If okay, pass up the name from the event
                    let name = data_transfer.get_data("field-name");
                    debug!("field name dropped - {}", name);
                    self.state.selected_field = Some(name);
                    self.link.send_message(Msg::OnChange);
                }

                true
            }
            Msg::OnDragover(event) => {
                // Prevent default to allow dropping here - if the types are correct
                let data_transfer = event.data_transfer().expect("No data transfer");

                // Verify type contraints
                let ftype = data_transfer.get_data("field-type");
                if check_compatible_types(&ftype, &self.state.ftype) {
                    event.prevent_default();
                }
                false
            }
            _ => false
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
        let field_value = match &self.state.selected_field {
            Some(name) => name.clone(),
            None => "".to_string()
        };

        html! {
            <>
                <td>{&self.state.name}</td>
                <td>{&self.state.ftype}</td>
                <td>
                    <input 
                        type="text" 
                        value=field_value
                        onchange=self.link.callback(|_| Msg::Nop)
                        ondrop=self.link.callback(|event| Msg::OnDrop(event))
                        ondragover=self.link.callback(|event| Msg::OnDragover(event))
                    />
                </td>
            </>
        }
    }
}

/*
*/
