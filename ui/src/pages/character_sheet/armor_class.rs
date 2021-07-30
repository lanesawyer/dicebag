use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

use crate::components::Icon;

#[derive(Properties, Clone, PartialEq)]
pub struct ArmorClassProps {
    pub value: i64,
}

pub struct ArmorClass {
    pub props: ArmorClassProps,
}

impl Component for ArmorClass {
    type Message = ();
    type Properties = ArmorClassProps;

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
            <section id="armor-class" class="text-block">
                <h3>{ "Armor Class" }</h3>
                <div>
                    <Icon name="shield" />
                    { &self.props.value }
                </div>
            </section>
        }
    }
}
