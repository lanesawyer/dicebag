use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

#[derive(Properties, Clone)]
pub struct DeathSavingThrows {
    pub saves: usize,
    pub failures: usize,
}

impl Component for DeathSavingThrows {
    type Message = ();
    type Properties = DeathSavingThrows;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {
            saves: props.saves,
            failures: props.failures,
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
            <section id="death-saving-throws" class="text-block">
                <h3>{ "Death Saving Throws" }</h3>
                <div>{ self.saves }</div>
                <div>{ self.failures }</div>
            </section>
        }
    }
}