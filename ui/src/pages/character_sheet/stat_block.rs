use crate::utils::calculate_modifier_display;
use yew::{classes, function_component, html, Html, Properties};

#[derive(PartialEq, Clone)]
pub enum Stat {
    Strength,
    Dexterity,
    Constitution,
    Intelligence,
    Wisdom,
    Charisma,
}

#[derive(Properties, Clone, PartialEq)]
pub struct StatBlockProps {
    pub name: String,
    pub value: i64,
    pub stat: Stat,
}

#[function_component(StatBlock)]
pub fn stat_block(props: &StatBlockProps) -> Html {
    html! {
        <div class={classes!("stat-block", stat_class(&props.stat))}>
            <div class="stat-name">{ &props.name }</div>
            <div class="stat-value">{ props.value }</div>
            <div class="stat-modifier">{ calculate_modifier_display(props.value) }</div>
        </div>
    }
}

fn stat_class(stat: &Stat) -> String {
    match stat {
        Stat::Strength => "strength".to_string(),
        Stat::Dexterity => "dexterity".to_string(),
        Stat::Constitution => "constitution".to_string(),
        Stat::Intelligence => "intelligence".to_string(),
        Stat::Wisdom => "wisdom".to_string(),
        Stat::Charisma => "charisma".to_string(),
    }
}
