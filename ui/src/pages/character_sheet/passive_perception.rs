use yew::{function_component, html, Html, Properties};

use crate::components::Icon;

#[derive(Properties, Clone, PartialEq)]
pub struct PassivePerceptionProps {
    pub value: i64,
}

#[function_component(PassivePerception)]
pub fn passive_perception(props: &PassivePerceptionProps) -> Html {
    html! {
        <section id="passive-perception" class="text-block">
            <h3>{ "Passive Perception" }</h3>
            <div>
                <Icon name="eye" />
                { props.value }
            </div>
        </section>
    }
}
