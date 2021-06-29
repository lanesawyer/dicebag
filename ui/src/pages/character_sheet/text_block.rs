use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

#[derive(Properties, Clone)]
pub struct TextBlock {
    pub name: String,
    pub value: String,
}

impl Component for TextBlock {
    type Message = ();
    type Properties = TextBlock;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {
            name: props.name,
            value: props.value,
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
                <h3>{ &self.name }</h3>
                <div>{ &self.value }</div>
            </section>
        }
    }
}
