use crate::components::{skill_display, Skill};
use yew::{function_component, html, Properties, Html};

#[derive(Properties, Clone, PartialEq)]
pub struct SavingThrowsProps {
    pub items: Vec<Skill>,
}

#[function_component(SavingThrows)]
pub fn savings_throws(props: &SavingThrowsProps) -> Html {
    html! {
        <section id="saving-throws" class="text-block">
            <h3>{"Saving Throws"}</h3>
            <ul class="skill-list">
                { props.items.iter().map(skill_display).collect::<Html>() }
            </ul>
        </section>
    }
}
