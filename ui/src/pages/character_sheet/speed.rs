use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

use crate::components::Icon;

#[derive(Properties, Clone, PartialEq)]
pub struct SpeedProps {
    pub value: i64,
}

pub struct Speed {
    pub props: SpeedProps,
}

impl Component for Speed {
    type Message = ();
    type Properties = SpeedProps;

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
            <section id="speed" class="text-block">
                <h3>{ "Speed" }</h3>
                <div>
                    <Icon name="flash" />
                    { self.props.value }
                </div>
            </section>
        }
    }
}
