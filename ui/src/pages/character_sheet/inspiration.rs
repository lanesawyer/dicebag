use yew::{function_component, html, Html, Properties};

use crate::components::Icon;

#[derive(Properties, Clone, PartialEq)]
pub struct InspirationProps {
    pub value: bool,
}

#[function_component(Inspiration)]
pub fn inpsiration(props: &InspirationProps) -> Html {
    html! {
        <section id="inspiration-section" class="single-value card">
            <Icon name="star" />
            <input type="checkbox" id="inspiration" name="inspiration" class="stat-value" checked={props.value} />
            <label for="inspiration">{ "Inspiration" }</label>
        </section>
    }
}
