use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

use crate::utils::{level_xp, level_xp_display};

#[derive(Properties, Clone, PartialEq)]
pub struct CharacterInfoProps {
    pub image: Option<String>,
    pub name: String,
    pub class: String, // TODO: enum
    pub level: i64,
    pub background: Option<String>,
    pub race: String,              // TODO: enum?
    pub alignment: Option<String>, // TODO: enum
    pub experience_points: i64,
}

pub struct CharacterInfo {
    pub props: CharacterInfoProps,
}

impl Component for CharacterInfo {
    type Message = ();
    type Properties = CharacterInfoProps;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        html! {
            <section id="character-info" class="text-block">
                <img src=self.props.image.clone() style={"width: 4em"}/>
                <h2>{ &self.props.name }</h2>
                <div>
                    <span>{ "Class" }</span>
                    <span>{ &self.props.class }</span>
                </div>
                <div>
                    <span>{ "Level" }</span>
                    <span>{ self.props.level }</span>
                </div>
                <div>
                    <span>{ "Background" }</span>
                    <span>{ self.props.background.as_ref().unwrap_or(&"".to_string()) }</span>
                </div>
                <div>
                    <span>{ "Race" }</span>
                    <span>{ &self.props.race }</span>
                </div>
                <div>
                    <span>{ "Alignment" }</span>
                    <span>{ self.props.alignment.as_ref().unwrap_or(&"".to_string()) }</span>
                </div>
                <span>
                    <label for="xp">{ "Experience Points:" }</label>
                    <span>{ level_xp_display(self.props.level) }</span>
                    <meter id="xp" min=0 max=1 value=self.calc_hp_bar_value()> { self.props.experience_points }</meter>
                    <span>{ level_xp_display(self.props.level + 1) }</span>
                </span>
            </section>
        }
    }
}

impl CharacterInfo {
    // The meter tag behaves poorly when the min is something like 900
    // so I normalize it because 0 to 1 on the meter works great.
    fn calc_hp_bar_value(&self) -> String {
        let previous_level_xp = level_xp(self.props.level);
        let next_level_xp = level_xp(self.props.level + 1);
        let current_xp = self.props.experience_points;
        
        let bar_value = (current_xp - previous_level_xp) as f64 / (next_level_xp - previous_level_xp) as f64;

        bar_value.to_string()
    }
}
