use yew::{function_component, html, Properties};

#[derive(Properties, Clone, PartialEq)]
pub struct HitDiceProps {
    pub total: i64,
    pub used: i64,
}

#[function_component(HitDice)]
pub fn view(props: &HitDiceProps) -> Html {
    html! {
        <section id="hit-dice" class="text-block">
            <h3>{ "Hit Dice" }</h3>
            <div>{ props.total }</div>
            <div>{ props.used }</div>
        </section>
    }
}
