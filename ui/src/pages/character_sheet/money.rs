use yew::{function_component, html, Html, Properties};

#[derive(Properties, Clone, PartialEq)]
pub struct MoneyProps {
    pub copper: i64,
    pub silver: i64,
    pub electrum: i64,
    pub gold: i64,
    pub platinum: i64,
}

#[function_component(Money)]
pub fn money(props: &MoneyProps) -> Html {
    html! {
        <section id="money" class="text-block">
            <div>{ format!("Copper: {}", props.copper) }</div>
            <div>{ format!("Silver: {}", props.silver) }</div>
            <div>{ format!("Electrum: {}", props.electrum) }</div>
            <div>{ format!("Gold: {}", props.gold) }</div>
            <div>{ format!("Platinum: {}", props.platinum) }</div>
        </section>
    }
}
