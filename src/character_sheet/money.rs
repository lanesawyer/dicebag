use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

#[derive(Properties, Clone)]
pub struct Money {
    pub copper: usize,
    pub silver: usize,
    pub electrum: usize,
    pub gold: usize,
    pub platinum: usize,
}

impl Component for Money {
    type Message = ();
    type Properties = Money;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {
            copper: props.copper,
            silver: props.silver,
            electrum: props.electrum,
            gold: props.electrum,
            platinum: props.platinum,
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
            <section id="money" class="text-block">
                <div>{ self.copper }</div>
                <div>{ self.silver }</div>
                <div>{ self.electrum }</div>
                <div>{ self.gold }</div>
                <div>{ self.platinum }</div>
            </section>
        }
    }
}
