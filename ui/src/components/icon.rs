use yew::prelude::*;
use yew::{function_component, html, Html};

#[derive(Properties, Clone, PartialEq)]
pub struct IconProps {
    pub name: String,
}

#[function_component(Icon)]
pub fn icon(props: &IconProps) -> Html {
    html! {
        <img src={format!("/assets/icons/{}.svg", props.name)} alt={props.name.clone()} class="icon" />
    }
}
