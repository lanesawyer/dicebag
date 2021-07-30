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
    html! {
        <li class="skill-display">
            <input type="checkbox" class="skill-proficiency" checked=skill.has_proficiency disabled=true />
            <span class="skill-modifier">{calculate_modifier_display(skill.ability_score)}</span>
            <span class=classes!("skill-name", stat_color_class(&skill.name))>{&skill.name}</span>
            <span class=classes!("related-ability")>
                {
                    if let Some(related_ability) = &skill.related_ability {
                        format!("({})", related_ability)
                    } else {
                        "".to_string()
                    }
                }
            </span>
        </li>
    }
}

fn stat_color_class(stat: &String) -> String {
    match stat.as_ref() {
        "Strength" => "strength-text".to_string(),
        "Dexterity" => "dexterity-text".to_string(),
        "Constitution" => "constitution-text".to_string(),
        "Intelligence" => "intelligence-text".to_string(),
        "Wisdom" => "wisdom-text".to_string(),
        "Charisma" => "charisma-text".to_string(),
        _ => "".to_string(),
    }
}
