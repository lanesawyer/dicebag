use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

#[derive(Properties, Clone)]
pub struct HitPoints {
    pub maximum: usize,
    pub current: usize,
    pub temporary: usize,
}

impl Component for HitPoints {
    type Message = ();
    type Properties = HitPoints;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {
            maximum: props.maximum,
            current: props.current,
            temporary: props.temporary,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <section id="hit-points" class="text-block">
                <h3>{ "Hit Points" }</h3>
                <div>{ format!("Maximum: {}", self.maximum) }</div>
                <div>{ format!("Current: {}", self.current) }</div>
                <div>{ format!("Temporary: {}", self.temporary) }</div>
            </section>
        }
    }
}
