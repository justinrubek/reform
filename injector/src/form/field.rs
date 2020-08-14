use failure::Error;

use yew::components::Select;
use yew::html::onchange::Event;
use yew::prelude::*;

#[derive(Clone, PartialEq, Deserialize)]
pub enum Type {
    Number,
    Text,
    Choice,
}

#[derive(Clone, PartialEq, Deserialize)]
pub struct FormField {
    pub name: String,
    pub ftype: Type,
    pub label: String,
    pub data: Option<Vec<String>>,
}

pub struct Field {
    link: ComponentLink<Self>,
    state: State,
    onchange: Callback<String>,
}

struct State {
    field: FormField,
    value: String,
}

pub enum Msg {
    UpdateField(String),
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[props(required)]
    pub field: FormField,
    #[props(required)]
    pub onchange: Callback<String>,
    #[props(required)]
    pub value: String,
}

impl Component for Field {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let state = State { 
            field: props.field,
            value: props.value,
        };

        Field {
            link,
            state,
            onchange: props.onchange,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateField(data) => {
                self.onchange.emit(data);
                true
            }
            _ => true,
        }
    }

    fn view(&self) -> Html {
        let input_field = match self.state.field.ftype {
            Type::Number => {
                html!{
                    <input 
                        type="number" 
                        onchange=self.link.callback(|event: Event| {
                            if let ChangeData::Value(val) = event {
                                Msg::UpdateField(val)
                            } else {
                                panic!("Onchange value not a string");
                            }
                        })
                        value=self.state.value />
                }
            }
            Type::Text => {
                html!{
                    <input type="text" 
                        onchange=self.link.callback(|event: Event| {
                            if let ChangeData::Value(val) = event {
                                Msg::UpdateField(val)
                            } else {
                                panic!("Onchange value not a string");
                            }
                        })
                        value=self.state.value 
                        class="input"/>
                }
            }
            Type::Choice => {
                let options = self.state.field.data.as_ref().unwrap();

                html!{
                    <Select<String> options=options onchange=self.link.callback(|v| Msg::UpdateField(v)) />
                }
            }
        };

        html! {
            <div>
                <label class="label">{&self.state.field.label}</label>
                {input_field}
            </div>
        }
    }
}
