use yew::prelude::*;
use yew::{html, Html};

pub struct Button {
    props: ButtonProps,
    link: ComponentLink<Self>,
}

#[derive(Clone, PartialEq)]
pub enum ButtonType {
    Primary,
    Success,
    Danger,
}

pub enum ButtonMsg {
    Clicked,
}

#[derive(Properties, Clone, PartialEq)]
pub struct ButtonProps {
    pub label: String,
    pub on_click: Callback<bool>,
    #[prop_or(ButtonType::Primary)]
    pub button_type: ButtonType,
}

impl Component for Button {
    type Message = ButtonMsg;
    type Properties = ButtonProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            ButtonMsg::Clicked => self.props.on_click.emit(true),
        }

        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
            <button type="button" class=classes!(self.choose_class()) onclick=self.link.callback(|_| ButtonMsg::Clicked)>
                { &self.props.label }
            </button>
        }
    }
}

impl Button {
    // chose class based on button type
    fn choose_class(&self) -> String {
        match self.props.button_type {
            ButtonType::Primary => "btn-primary".to_string(),
            ButtonType::Success => "btn-success".to_string(),
            ButtonType::Danger => "btn-danger".to_string(),
        }
    }
}
