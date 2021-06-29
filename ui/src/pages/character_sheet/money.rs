use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

#[derive(Properties, Clone, PartialEq)]
pub struct MoneyProps {
    pub copper: i64,
    pub silver: i64,
    pub electrum: i64,
    pub gold: i64,
    pub platinum: i64,
}

pub struct Money {
    pub props: MoneyProps,
}

impl Component for Money {
    type Message = ();
    type Properties = MoneyProps;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        html! {
            <section id="money" class="text-block">
                <div>{ format!("Copper: {}", self.props.copper) }</div>
                <div>{ format!("Silver: {}", self.props.silver) }</div>
                <div>{ format!("Electrum: {}", self.props.electrum) }</div>
                <div>{ format!("Gold: {}", self.props.gold) }</div>
                <div>{ format!("Platinum: {}", self.props.platinum) }</div>
            </section>
        }
    }
}
