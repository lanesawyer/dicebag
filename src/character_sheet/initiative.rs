use crate::utils::calculate_modifier_display;
use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

#[derive(Properties, Clone)]
pub struct Initiative {
    pub value: usize,
}

impl Component for Initiative {
    type Message = ();
    type Properties = Initiative;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { value: props.value }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <section class="text-block">
                <h2>{ "Initiative" }</h2>
                <div>{ calculate_modifier_display(self.value) }</div>
            </section>
        }
    }
}
