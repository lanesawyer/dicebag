use yew::{function_component, html, Properties};

use crate::components::Icon;

#[derive(Properties, Clone, PartialEq)]
pub struct HitPointsProps {
    pub maximum: i64,
    pub current: i64,
    pub temporary: i64,
}

#[function_component(HitPoints)]
pub fn hit_points(props: &HitPointsProps) -> Html {
    html! {
        <section id="hit-points" class="text-block">
            <h3>{ "Hit Points" }</h3>
            <Icon name="pulse" />
            <span>{ "0" }</span>
            <meter
                min="0"
                low={calc_low_hp(props.maximum)}
                max={props.maximum.to_string()}
                value={props.current.to_string()}></meter>
            <span>{ props.maximum }</span>
            <div>{ format!("Current: {}", props.current) }</div>
            // TODO: Figure out extra bar to show temporary hit points
            <div>{ format!("Temporary: {}", props.temporary) }</div>
        </section>
    }
}

fn calc_low_hp(maximum: i64) -> String {
    (maximum as f64 * 0.50).to_string()
}
