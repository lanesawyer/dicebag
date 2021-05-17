use crate::components::{Skill, skill_display};
use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

#[derive(Properties, Clone)]
pub struct SavingThrows {
    pub items: Vec<Skill>,
}

impl Component for SavingThrows {
    type Message = ();
    type Properties = SavingThrows;

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
            <section id="saving-throws" class="text-block">
                <h3>{"Saving Throws"}</h3>
                <ul>
                { self.items.iter().map(skill_display).collect::<Html>() }
                </ul>
            </section>
        }
    }
}
