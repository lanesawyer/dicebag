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
                <h4>{ "Successes" }</h4>
                <input type="checkbox" checked={self.saves >= 1} />
                <input type="checkbox" checked={self.saves >= 2} />
                <input type="checkbox" checked={self.saves >= 3}/>

                <h4>{ "Failures" }</h4>
                <input type="checkbox" checked={self.failures >= 1} />
                <input type="checkbox" checked={self.failures >= 2} />
                <input type="checkbox" checked={self.failures >= 3} />
            </section>
        }
    }
}
