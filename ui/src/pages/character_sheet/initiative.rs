use crate::utils::calculate_modifier_display;
use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

#[derive(Properties, Clone, PartialEq)]
pub struct InitiativeProps {
    pub value: i64,
}

pub struct Initiative {
    pub props: InitiativeProps,
}

impl Component for Initiative {
    type Message = ();
    type Properties = InitiativeProps;

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
            <section class="text-block">
                <h3>{ "Initiative" }</h3>
                <div>{ calculate_modifier_display(self.props.value) }</div>
            </section>
        }
    }
}
