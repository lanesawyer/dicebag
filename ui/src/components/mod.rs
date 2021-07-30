use crate::utils::calculate_modifier_display;
use yew::{classes, html, Html};

mod button;
mod icon;
mod text_field;

pub use button::{Button, ButtonType};
pub use icon::Icon;
pub use text_field::TextField;

#[derive(Clone, PartialEq, PartialOrd)]
pub struct Skill {
    pub has_proficiency: bool,
    pub ability_score: i64,
    pub name: String,
    pub related_ability: Option<String>,
}

pub fn skill_display(skill: &Skill) -> Html {
    let related_ability_node = if let Some(related_ability) = &skill.related_ability {
        html! { 
            <span class=classes!("related-ability", stat_color_class(related_ability))>
                { format!("({})", related_ability) }
            </span>
        }
        } else {
            html! { <></> }
        };

    html! {
        <li class="skill-display">
            <input type="checkbox" class="skill-proficiency" checked=skill.has_proficiency disabled=true />
            <span class="skill-modifier">{calculate_modifier_display(skill.ability_score)}</span>
            <span class=classes!("skill-name", stat_color_class(&skill.name))>{&skill.name}</span>
            {
                related_ability_node
            }
        </li>
    }
}

fn stat_color_class(stat: &str) -> String {
    match stat {
        "Strength" | "Str" => "strength-text".to_string(),
        "Dexterity" | "Dex" => "dexterity-text".to_string(),
        "Constitution" | "Con" => "constitution-text".to_string(),
        "Intelligence" | "Int" => "intelligence-text".to_string(),
        "Wisdom" | "Wis" => "wisdom-text".to_string(),
        "Charisma" | "Cha" => "charisma-text".to_string(),
        _ => "".to_string(),
    }
}
