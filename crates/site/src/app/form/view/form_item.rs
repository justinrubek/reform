
use yew::prelude::*;




use crate::types::FormInfo;

pub struct FormItem {
    state: State,
    link: ComponentLink<Self>,
    props: Props,
}

struct State {
}

pub enum Msg {
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub form: FormInfo,
}


impl Component for FormItem {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let state = State {
        };

        FormItem { 
            state,
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
            <div class="media">
                <div class="media-left">
                    <p>{format!("id: {}", self.props.form.id)}</p>
                </div>
                <div class="media-left">
                    <p>{format!("name: {}", self.props.form.name)}</p>
                </div>
                <div class="media-right">
                    // <button class="button" >{"edit"}</button> // Currently no form editing
                </div>
            </div>
        }
    }
}
