use crate::components::{skill_display, Skill};
use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

#[derive(Properties, Clone, PartialEq)]
pub struct SavingThrowsProps {
    pub items: Vec<Skill>,
}

pub struct SavingThrows {
    pub props: SavingThrowsProps,
}

impl Component for SavingThrows {
    type Message = ();
    type Properties = SavingThrowsProps;

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
            <section id="saving-throws" class="text-block">
                <h3>{"Saving Throws"}</h3>
                <ul class="skill-list">
                    { self.props.items.iter().map(skill_display).collect::<Html>() }
                </ul>
            </section>
        }
    }
}
