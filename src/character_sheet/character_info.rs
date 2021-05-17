use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

#[derive(Properties, Clone)]
pub struct CharacterInfo {
    pub name: String,
    pub class: String, // TODO: enum
    pub level: usize,
    pub background: String,
    pub race: String,      // TODO: enum?
    pub alignment: String, // TODO: enum
    pub experience_points: usize,
}

impl Component for CharacterInfo {
    type Message = ();
    type Properties = CharacterInfo;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {
            name: props.name,
            class: props.class,
            level: props.level,
            background: props.background,
            race: props.race,
            alignment: props.alignment,
            experience_points: props.experience_points,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <section id="character-info" class="text-block">
                <h2>{ &self.name }</h2>
                <span>{ format!("Class: {}", &self.class) }</span>
                <span>{ format!("Level: {}", self.level) }</span>
                <span>{ format!("Background: {}", &self.background) }</span>
                <span>{ format!("Race: {}", &self.race) }</span>
                <span>{ format!("Alignment: {}", &self.alignment) }</span>
                <span>{ format!("Experience Points: {}", self.experience_points) }</span>
            </section>
        }
    }
}
