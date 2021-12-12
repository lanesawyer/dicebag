use crate::utils::calculate_modifier_display;
use yew::{function_component, html, Html, Properties};

#[derive(Properties, Clone, PartialEq)]
pub struct InitiativeProps {
    pub value: i64,
}

#[function_component(Initiative)]
pub fn initiative(props: &InitiativeProps) -> Html {
    html! {
        <section class="text-block">
            <h3>{ "Initiative" }</h3>
            <div>{ calculate_modifier_display(props.value) }</div>
        </section>
    }
}
