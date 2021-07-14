use crate::utils::calculate_modifier_display;
use yew::{html, Html};

mod button;
mod text_field;

pub use button::Button;
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
            <span class="skill-name">{&skill.name}</span>
            <span class="related-ability">
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
