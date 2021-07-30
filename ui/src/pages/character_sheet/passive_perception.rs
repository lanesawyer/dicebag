use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

use crate::components::Icon;

#[derive(Properties, Clone, PartialEq)]
pub struct PassivePerceptionProps {
    pub value: i64,
}

pub struct PassivePerception {
    pub props: PassivePerceptionProps,
}

impl Component for PassivePerception {
    type Message = ();
    type Properties = PassivePerceptionProps;

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
            <section id="passive-perception" class="text-block">
                <h3>{ "Passive Perception" }</h3>
                <div>
                    <Icon name="eye" />
                    { self.props.value }
                </div>
            </section>
        }
    }
}
