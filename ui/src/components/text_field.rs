use yew::prelude::*;
use yew::{html, Html};

pub struct TextField {
    props: TextFieldProps,
    link: ComponentLink<Self>,
}

pub enum TextFieldMsg {
    ValueChanged(String),
}

#[derive(Properties, Clone, PartialEq)]
pub struct TextFieldProps {
    pub label: String,
    pub value: String,
    pub on_change: Callback<String>,
}

impl Component for TextField {
    type Message = TextFieldMsg;
    type Properties = TextFieldProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            TextFieldMsg::ValueChanged(new_value) => self.props.on_change.emit(new_value),
        }

        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <label>{ &self.props.label }</label>
                <input
                    type="text"
                    value=self.props.value.clone()
                    oninput=self.link.callback(|e: InputData| TextFieldMsg::ValueChanged(e.value))
                />
            </div>
        }
    }
}
