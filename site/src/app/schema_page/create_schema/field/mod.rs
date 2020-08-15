use yew::components::Select;
use yew::format::Json;
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Response, Request};

#[derive(Clone, Copy, PartialEq)]
pub enum Type {
    Number,
    Text,
}

impl Default for Type {
    fn default() -> Self {
        Type::Text
    }
}

impl ToString for Type {
    fn to_string(&self) -> String {
        match self {
            Type::Number => "Number".into(),
            Type::Text => "Text".into(),
        }
    }
}

#[derive(Clone, Default, PartialEq)]
pub struct Field {
    pub name: String,
    pub ftype: Type,
}

struct State {
    field: Field,
}

pub enum Msg {
    OnChange,
    UpdateName(String),
    UpdateType(Type), 
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub onchange: Callback<Field>,
    pub field: Field,
}

pub struct FieldItem {
    state: State,
    link: ComponentLink<Self>,
    onchange: Callback<Field>,
}

impl Component for FieldItem {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let state = State {
            field: props.field,
        };

        FieldItem { 
            state: state,
            link,
            onchange: props.onchange,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::OnChange => {
                self.onchange.emit(self.state.field.clone());
                true
            }
            Msg::UpdateName(name) => {
                self.state.field.name = name;
                self.link.send_message(Msg::OnChange);
                true
            }
            Msg::UpdateType(ftype) => {
                self.state.field.ftype = ftype;
                self.link.send_message(Msg::OnChange);
                true
            }
            _ => false
        }
    }

    fn view(&self) -> Html {
        let type_options = vec![Type::Number, Type::Text];

        let name_field = html!{ 
            <input type="text"
                   value=self.state.field.name 
                   name="name" 
                   oninput=self.link.callback(|e: InputData| Msg::UpdateName(e.value))
             />
        };

        let type_field = html!{ 
            <Select<Type> options=type_options onchange=self.link.callback(|v| Msg::UpdateType(v)) />
        };


        html! {
            <tr>
                <td>{name_field}</td>
                <td>{type_field}</td>
            </tr>
        }
    }
}

