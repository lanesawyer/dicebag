use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew::{function_component, html};

#[derive(Properties, Clone, PartialEq)]
pub struct TextFieldProps {
    pub label: String,
    pub value: String,
    pub on_change: Callback<String>,
}

#[function_component(TextField)]
pub fn text_field(props: &TextFieldProps) -> Html {
    let TextFieldProps {
        label,
        value,
        on_change,
    } = props.clone();

    let oninput = Callback::from(move |input_event: InputEvent| {
        on_change.emit(get_value_from_input_event(input_event));
    });

    html! {
        <div class="text-field">
            <label>{ &label }</label>
            <input
                type="text"
                {value}
                {oninput}
            />
        </div>
    }
}

fn get_value_from_input_event(e: InputEvent) -> String {
    let event: Event = e.dyn_into().unwrap_throw();
    let event_target = event.target().unwrap_throw();
    let target: HtmlInputElement = event_target.dyn_into().unwrap_throw();
    target.value()
}
