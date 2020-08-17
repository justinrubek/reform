use failure::Error;

use yew::html::onchange::Event;
use yew::prelude::*;

// TODO: This is also supplied by the site crate. The provided one on up to date yew requires
// web_sys which we do not support here
use yew_components::Select;

#[derive(Clone, PartialEq, Deserialize)]
pub enum Type {
    Number,
    Text,
    Choice,
}

pub enum FieldValue {

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
    onchange: Callback<serde_json::Value>,
    props: Props,
}

struct State {
    field: FormField,
    value: serde_json::Value,
}

pub enum Msg {
    UpdateField(serde_json::Value),
    InvalidInput,
    NoOp,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub field: FormField,
    pub onchange: Callback<serde_json::Value>,
    pub value: serde_json::Value,
}

impl Component for Field {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let state = State { 
            field: props.field.clone(),
            value: props.value.clone(),
        };

        Field {
            link,
            state,
            onchange: props.onchange.clone(),
            props: props,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateField(data) => {
                self.state.value = data.clone();
                self.onchange.emit(data);
                true
            }
            Msg::InvalidInput => {
                true
            }
            _ => true,
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
        let input_field = match self.state.field.ftype {
            Type::Number => {
                let value = self.state.value.clone();
                let callback = self.link.callback(move |event: InputData| {
                    info!("Number changed");
                    match event.value.parse::<isize>() {
                        Ok(integer) => Msg::UpdateField(json!(integer)),
                        _ => match event.value.parse::<f64>() {
                            Ok(float) => {
                                Msg::UpdateField(json!(float))
                            }
                            _ => Msg::InvalidInput
                        }
                    }
                });

                html! {
                    <input 
                        type="number" 
                        name=self.state.field.name
                        oninput=callback
                        value=self.state.value />
                }
            }
            Type::Text => {
                if let serde_json::Value::String(value) = &self.state.value {
                    html!{
                        <input type="text" name=self.state.field.name 
                            onchange=self.link.callback(|event: Event| {
                                if let ChangeData::Value(val) = event {
                                    Msg::UpdateField(json!(val))
                                } else {
                                    panic!("Onchange value not a string");
                                }
                            })
                            value=self.state.value.as_str().expect("No string value for text")
                            class="input"/>
                    }
                } else {
                    unreachable!();
                }
            }
            Type::Choice => {
                let options = self.state.field.data.as_ref().unwrap();

                html!{
                    <Select<String> options=options on_change=self.link.callback(|v| Msg::UpdateField(json!(v))) />
                }
            }
        };

        html! {
            <div>
                <label class="label" for=self.state.field.name>{&self.state.field.label}</label>
                {input_field}
            </div>
        }
    }
}
