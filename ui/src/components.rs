use crate::utils::calculate_modifier_display;
use yew::{html, Html};

#[derive(Clone)]
pub struct Skill {
    pub has_proficiency: bool,
    pub ability_score: i64,
    pub name: String,
    pub related_ability: Option<String>,
}

pub fn skill_display(skill: &Skill) -> Html {
    html! {
        <li class="skill-display">
            <input type="checkbox" checked=skill.has_proficiency />
            <span>{calculate_modifier_display(skill.ability_score)}</span>
            <span>{&skill.name}</span>
            {
                if let Some(related_ability) = &skill.related_ability {
                    html! { <span>{format!("({})", related_ability)}</span> }
                } else {
                    html! { <></> }
                }
            }
        </li>
    }
}
