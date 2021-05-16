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
            <section class="text-block">
                <h2>{ "Hit Points" }</h2>
                <div>{ self.maximum }</div>
                <div>{ self.current }</div>
                <div>{ self.temporary }</div>
            </section>
        }
    }
}
