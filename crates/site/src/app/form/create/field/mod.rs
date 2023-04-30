use serde::Serialize;

use stdweb::traits::IDragEvent;

use yew::prelude::*;

use crate::components::Select;

mod choice_field;
use choice_field::ChoiceField;

#[derive(Clone, PartialEq, Serialize)]
pub enum Type {
    Number,
    Text,
    Choice,
}

impl Default for Type {
    fn default() -> Self {
        Type::Text
    }
}

impl ToString for Type {
    fn to_string(&self) -> String {
        match self {
            Type::Number => "number".into(),
            Type::Text => "text".into(),
            Type::Choice => "choice".into(),
        }
    }
}

#[derive(Clone, Default, PartialEq, Serialize)]
pub struct Field {
    name: String,
    ftype: Type,
    label: String,
    data: Option<Vec<String>>,
}

struct State {
    field: Field,
}

pub enum Msg {
    OnChange,
    UpdateName(String),
    UpdateType(Type),
    UpdateLabel(String),
    UpdateData(Vec<String>),
    StartDrag(DragStartEvent),
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub onchange: Callback<Field>,
    pub field: Field,
}

pub struct FieldItem {
    state: State,
    link: ComponentLink<Self>,
    props: Props,
}

impl Component for FieldItem {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let state = State {
            field: props.field.clone(),
        };

        FieldItem { state, link, props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::OnChange => {
                self.props.onchange.emit(self.state.field.clone());
                true
            }
            Msg::UpdateName(name) => {
                self.state.field.name = name;
                self.link.send_message(Msg::OnChange);
                true
            }
            Msg::UpdateLabel(label) => {
                self.state.field.label = label;
                self.link.send_message(Msg::OnChange);
                true
            }
            Msg::UpdateType(ftype) => {
                self.state.field.ftype = ftype;
                self.link.send_message(Msg::OnChange);
                true
            }
            Msg::UpdateData(data) => {
                self.state.field.data = Some(data);
                self.link.send_message(Msg::OnChange);
                true
            }
            Msg::StartDrag(event) => {
                info!("drag started");
                // event.data_transfer().expect("Failed to create drag event").set_data("field-name", &self.state.field.label);
                let data_transfer = event.data_transfer().expect("Failed to create drag event");
                data_transfer.set_data("field-name", &self.state.field.name);
                data_transfer.set_data("field-type", &self.state.field.ftype.to_string());
                data_transfer.set_data("field-label", &self.state.field.label);
                true
            }
            _ => false,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.state.field = props.field.clone();
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let type_options = vec![Type::Number, Type::Text, Type::Choice];

        let name_field = html! {
            <input type="text"
                   value=self.state.field.name
                   name="name"
                   oninput=self.link.callback(|e: InputData| Msg::UpdateName(e.value))
             />
        };

        let type_field = html! {
            <Select<Type> options=type_options onchange=self.link.callback(|v| Msg::UpdateType(v)) />
        };

        let label_field = html! {
            <input type="text"
                   value=self.state.field.label
                   name="label"
                   oninput=self.link.callback(|e: InputData| Msg::UpdateLabel(e.value))
            />
        };

        let drag_hook = html! {
            <div draggable=true ondragstart=self.link.callback(|ev| Msg::StartDrag(ev)) class="drag-hook">
                {"o"}
            </div>
        };

        html! {
            <tr>
                <td>{name_field}</td>
                <td>{type_field}</td>
                <td>{label_field}</td>
                {
                    if self.state.field.ftype == Type::Choice {
                        html! {
                            <td>
                                <ChoiceField
                                    onchange=self.link.callback(|data: Vec<String>| Msg::UpdateData(data))
                                />
                            </td>
                        }
                    }
                    else {
                        html!{}
                    }
                }
                <td>{drag_hook}</td>
            </tr>
        }
    }
}
