use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

#[derive(Properties, Clone)]
pub struct Money {
    pub copper: i64,
    pub silver: i64,
    pub electrum: i64,
    pub gold: i64,
    pub platinum: i64,
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
                <div>{ format!("Copper: {}", self.copper) }</div>
                <div>{ format!("Silver: {}", self.silver) }</div>
                <div>{ format!("Electrum: {}", self.electrum) }</div>
                <div>{ format!("Gold: {}", self.gold) }</div>
                <div>{ format!("Platinum: {}", self.platinum) }</div>
            </section>
        }
    }
}
