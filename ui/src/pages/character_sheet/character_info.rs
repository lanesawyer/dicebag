use yew::{function_component, html, Properties};

use crate::{
    pages::character_sheet::text_block::TextBlock,
    utils::{level_xp, level_xp_display},
};

#[derive(Properties, Clone, PartialEq)]
pub struct CharacterInfoProps {
    pub image: Option<String>,
    pub name: String,
    pub class: String,
    pub level: i64,
    pub background: Option<String>,
    pub race: String,
    pub alignment: Option<String>,
    pub experience_points: i64,
}

#[function_component(CharacterInfo)]
pub fn character_info(props: &CharacterInfoProps) -> Html {
    html! {
        <section id="character-info" class="text-block">
            <img src={props.image.clone()} style={"width: 4em"}/>
            <div>
                <h2>{ &props.name }</h2>
                <span> { &props.race }</span>
                <span> { &props.class }</span>
                <span> { &props.level }</span>
                <div>
                    <label for="xp">{ "Experience Points:" }</label>
                    <span>{ level_xp_display(props.level) }</span>
                    <meter id="xp" min={0} max={1} value={calc_hp_bar_value(props.level, props.experience_points)}> { props.experience_points }</meter>
                    <span>{ level_xp_display(props.level + 1) }</span>
                </div>
            </div>
            <TextBlock name="Background" value={props.background.clone()} />
            <TextBlock name="Alignment" value={props.alignment.clone()} />
        </section>
    }
}

// The meter tag behaves poorly when the min is something like 900
// so I normalize it because 0 to 1 on the meter works great.
fn calc_hp_bar_value(level: i64, experience_points: i64) -> String {
    let previous_level_xp = level_xp(level);
    let next_level_xp = level_xp(level + 1);
    let current_xp = experience_points;

    let bar_value =
        (current_xp - previous_level_xp) as f64 / (next_level_xp - previous_level_xp) as f64;

    bar_value.to_string()
}
