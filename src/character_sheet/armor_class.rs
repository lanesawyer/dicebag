use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

#[derive(Properties, Clone)]
pub struct ArmorClass {
    pub value: usize,
}

impl Component for ArmorClass {
    type Message = ();
    type Properties = ArmorClass;

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
            <section id="armor-class" class="text-block">
                <h3>{ "Armor Class" }</h3>
                <div>{ &self.value }</div>
            </section>
        }
    }
}
