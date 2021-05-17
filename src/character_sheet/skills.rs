use crate::components::{Skill, skill_display};
use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};


#[derive(Properties, Clone)]
pub struct Skills {
    pub items: Vec<Skill>,
}

impl Component for Skills {
    type Message = ();
    type Properties = Skills;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { items: props.items }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <section id="skills" class="text-block">
                <h3>{ "Skills"}</h3>
                <ul>
                { self.items.iter().map(skill_display).collect::<Html>() }
                </ul>
            </section>
        }
    }
}
