use yew::prelude::*;
use yew::{html, Html};

use crate::components::Icon;

pub struct Button;

#[derive(Clone, PartialEq)]
pub enum ButtonType {
    Primary,
    // Success,
    Danger,
}

pub enum ButtonMsg {
    Clicked,
}

#[derive(Properties, Clone, PartialEq)]
pub struct ButtonProps {
    pub label: String,
    pub on_click: Callback<()>,
    #[prop_or(ButtonType::Primary)]
    pub button_type: ButtonType,
    #[prop_or_default]
    pub icon_name: Option<String>,
}

impl Component for Button {
    type Message = ButtonMsg;
    type Properties = ButtonProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ButtonMsg::Clicked => ctx.props().on_click.emit(()),
        }

        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <button type="button" class={classes!(self.choose_class(ctx))} onclick={ctx.link().callback(|_| ButtonMsg::Clicked)}>
                {
                    if let Some(icon_name) = &ctx.props().icon_name {
                        html! {
                            <Icon name={icon_name.clone()} />
                        }
                    } else {
                        html! { <></> }
                    }
                }
                { &ctx.props().label }
            </button>
        }
    }
}

impl Button {
    // chose class based on button type
    fn choose_class(&self, ctx: &Context<Self>) -> String {
        match ctx.props().button_type {
            ButtonType::Primary => "btn-primary".to_string(),
            // ButtonType::Success => "btn-success".to_string(),
            ButtonType::Danger => "btn-danger".to_string(),
        }
    }
}
