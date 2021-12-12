use yew::{function_component, html, Html, Properties};

use crate::components::Icon;

#[derive(Properties, Clone, PartialEq)]
pub struct SpeedProps {
    pub value: i64,
}

#[function_component(Speed)]
pub fn speed(props: &SpeedProps) -> Html {
    html! {
        <section id="speed" class="text-block">
            <h3>{ "Speed" }</h3>
            <div>
                <Icon name="flash" />
                { props.value }
            </div>
        </section>
    }
}
