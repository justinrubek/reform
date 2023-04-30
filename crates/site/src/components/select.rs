use yew::callback::Callback;
use yew::html::{ChangeData, Component, ComponentLink, Html, ShouldRender};
use yew::macros::{html, Properties};

/// `Select` component.
/// Taken and modified from yew v0.11.0 at https://github.com/yewstack/yew/blob/v0.11.0/src/components/select.rs
/// The update to date version of this provided by yew is in the crate yew-components which
/// requires web-sys. One day this site may support that, but currently it does not
#[derive(Debug)]
pub struct Select<T: ToString + PartialEq + Clone + 'static> {
    props: Props<T>,
    link: ComponentLink<Self>,
}

#[derive(Debug)]
pub enum Msg {
    Selected(Option<usize>),
}

#[derive(PartialEq, Clone, Properties, Debug)]
pub struct Props<T: Clone> {
    /// Initially selected value.
    #[prop_or_default]
    pub selected: Option<T>,
    /// Disabled the component's selector.
    #[prop_or_default]
    pub disabled: bool,
    /// Options are available to choose.
    #[prop_or_default]
    pub options: Vec<T>,
    /// Callback to handle changes.
    pub onchange: Callback<T>,
}

impl<T> Component for Select<T>
where
    T: ToString + PartialEq + Clone + 'static,
{
    type Message = Msg;
    type Properties = Props<T>;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Selected(value) => {
                if let Some(idx) = value {
                    let item = self.props.options.get(idx - 1).cloned();
                    if let Some(value) = item {
                        self.props.onchange.emit(value);
                    }
                }
            }
        }
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
        let selected = self.props.selected.as_ref();
        let view_option = |value: &T| {
            let flag = selected == Some(value);
            html! {
                <option selected=flag>{ value.to_string() }</option>
            }
        };

        html! {
            <select disabled=self.props.disabled onchange=self.onchange()>
                <option disabled=true selected=selected.is_none()>
                    { "↪" }
                </option>
                { for self.props.options.iter().map(view_option) }
            </select>
        }
    }
}

impl<T> Select<T>
where
    T: ToString + PartialEq + Clone + 'static,
{
    fn onchange(&self) -> Callback<ChangeData> {
        self.link.callback(|event| match event {
            ChangeData::Select(elem) => {
                let value = elem.selected_index().map(|x| x as usize);
                Msg::Selected(value)
            }
            _ => {
                unreachable!();
            }
        })
    }
}
