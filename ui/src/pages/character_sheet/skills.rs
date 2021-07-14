use crate::components::{skill_display, Skill};
use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

#[derive(Properties, Clone, PartialEq)]
pub struct SkillsProps {
    pub items: Vec<Skill>,
}

pub struct Skills {
    pub props: SkillsProps,
}

impl Component for Skills {
    type Message = ();
    type Properties = SkillsProps;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        html! {
            <section id="skills" class="text-block">
                <h3>{ "Skills"}</h3>
                <ul class="skill-list">
                { self.props.items.iter().map(skill_display).collect::<Html>() }
                </ul>
            </section>
        }
    }
}
