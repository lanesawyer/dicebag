use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

use crate::{
    pages::character_sheet::text_block::TextBlock,
    utils::{level_xp, level_xp_display},
};

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
                <div>
                    <h2>{ &self.props.name }</h2>
                    <span> { &self.props.race }</span>
                    <span> { &self.props.class }</span>
                    <span> { &self.props.level }</span>
                    <div>
                        <label for="xp">{ "Experience Points:" }</label>
                        <span>{ level_xp_display(self.props.level) }</span>
                        <meter id="xp" min=0 max=1 value=self.calc_hp_bar_value()> { self.props.experience_points }</meter>
                        <span>{ level_xp_display(self.props.level + 1) }</span>
                    </div>
                </div>
                <TextBlock name="Background" value=self.props.background.clone() />
                <TextBlock name="Alignment" value=self.props.alignment.clone() />
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

        let bar_value =
            (current_xp - previous_level_xp) as f64 / (next_level_xp - previous_level_xp) as f64;

        bar_value.to_string()
    }
}
