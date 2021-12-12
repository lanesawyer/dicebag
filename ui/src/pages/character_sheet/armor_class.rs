use yew::{function_component, html, Html, Properties};

use crate::components::Icon;

#[derive(Properties, Clone, PartialEq)]
pub struct ArmorClassProps {
    pub value: i64,
}

#[function_component(ArmorClass)]
pub fn armor_class(props: &ArmorClassProps) -> Html {
    html! {
        <section id="armor-class" class="text-block">
            <h3>{ "Armor Class" }</h3>
            <div>
                <Icon name="shield" />
                { props.value }
            </div>
        </section>
    }
}
