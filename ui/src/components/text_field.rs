use yew::prelude::*;
use yew::{html, Html};
use web_sys::HtmlInputElement as InputElement;

pub struct TextField;

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

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            TextFieldMsg::ValueChanged(new_value) => ctx.props().on_change.emit(new_value),
        }

        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onkeypress = ctx.link().batch_callback(|e: KeyboardEvent| {
            if e.key() == "Enter" {
                let input: InputElement = e.target_unchecked_into();
                let value = input.value();
                input.set_value("");
                Some(TextFieldMsg::ValueChanged(value))
            } else {
                None
            }
        });

        html! {
            <div class="text-field">
                <label>{ &ctx.props().label }</label>
                <input
                    type="text"
                    value={ctx.props().value.clone()}
                    {onkeypress}
                />
            </div>
        }
    }
}
