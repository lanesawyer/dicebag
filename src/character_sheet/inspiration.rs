use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

#[derive(Properties, Clone)]
pub struct Inspiration {
    pub value: bool,
}

impl Component for Inspiration {
    type Message = ();
    type Properties = Inspiration;

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
            <section id="inspiration" class="stat-block">
                <div class="stat-name">{ "Inspiration" }</div>
                <input type="checkbox" class="stat-value" checked=self.value />
            </section>
        }
    }
}
