use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

#[derive(Properties, Clone)]
pub struct Speed {
    pub value: usize,
}

impl Component for Speed {
    type Message = ();
    type Properties = Speed;

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
            <section id="speed" class="text-block">
                <h2>{ "Speed" }</h2>
                <div>{ self.value }</div>
            </section>
        }
    }
}
