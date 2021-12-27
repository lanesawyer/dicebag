use crate::components::{skill_display, Skill};
use yew::{function_component, html, Properties, Html};

#[derive(Properties, Clone, PartialEq)]
pub struct SkillsProps {
    pub items: Vec<Skill>,
}

#[function_component(Skills)]
pub fn skills(props: &SkillsProps) -> Html {
    html! {
        <section id="skills" class="text-block">
            <h3>{ "Skills"}</h3>
            <ul class="skill-list">
            { props.items.iter().map(skill_display).collect::<Html>() }
            </ul>
        </section>
    }
}
