use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

#[derive(Properties, Clone)]
pub struct Skills {
    pub name: String,
    pub value: String,
}

impl Component for Skills {
    type Message = ();
    type Properties = Skills;

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
            <section id="skills" class="text-block">
                <div>{ &self.name }</div>
                <div>{ &self.value }</div>
            </section>
        }
    }
}
