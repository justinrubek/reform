use yew::format::Json;
use yew::prelude::*;

use crate::components::Select;

struct State {
    choices: Vec<String>,
}

pub enum Msg {
    OnChange,
    AddChoice,
    RemoveChoice(usize),
    UpdateChoice(usize, String),
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub onchange: Callback<Vec<String>>,
}

pub struct ChoiceField {
    state: State,
    link: ComponentLink<Self>,
    props: Props,
}

impl Component for ChoiceField {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let state = State {
            choices: Vec::new(),
        };

        ChoiceField { 
            state: state,
            link,
            props
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::OnChange => {
                self.props.onchange.emit(self.state.choices.clone());
                true
            }
            Msg::AddChoice => {
                self.state.choices.push("".into());
                self.link.send_message(Msg::OnChange);
                true
            }
            Msg::RemoveChoice(index) => {
                self.state.choices.remove(index);
                self.link.send_message(Msg::OnChange);
                true
            }
            Msg::UpdateChoice(index, value) => {
                self.state.choices[index] = value;
                self.link.send_message(Msg::OnChange);
                true
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
        let choices = self.state.choices.iter().enumerate().map(|(i, choice)| {
            html! {
                <div>
                    <input type="text"
                           value=choice
                           name="choice" 
                           oninput=self.link.callback(move |e: InputData| Msg::UpdateChoice(i, e.value))
                    />
                    <button class="button media-right" onclick=self.link.callback(move |_| Msg::RemoveChoice(i))>{"x"}</button>
                </div>
            }
        }).collect::<Html>();

        html! {
            <p>
            {choices}
            <button class="button media-right" onclick=self.link.callback(|_| Msg::AddChoice)>{"add choice"}</button>
            </p>
        }
    }
}

