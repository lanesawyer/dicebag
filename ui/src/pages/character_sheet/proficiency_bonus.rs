use yew::{function_component, html, Properties};

#[derive(Properties, Clone, PartialEq)]
pub struct ProficiencyBonusProps {
    pub value: i64,
}

#[function_component(ProficiencyBonus)]
pub fn proficiency_bonus(props: &ProficiencyBonusProps) -> Html {
    html! {
        <section id="proficiency-bonus" class="text-block">
            <h3>{ "Proficiency Bonus" }</h3>
            <div>{ props.value }</div>
        </section>
    }
}
