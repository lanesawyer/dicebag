use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

#[derive(Properties, Clone, PartialEq)]
pub struct CharacterInfoProps {
    pub image: String,
    pub name: String,
    pub class: String, // TODO: enum
    pub level: i64,
    pub background: String,
    pub race: String,      // TODO: enum?
    pub alignment: String, // TODO: enum
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
                <span>{ format!("Class: {}", &self.props.class) }</span>
                <span>{ format!("Level: {}", self.props.level) }</span>
                <span>{ format!("Background: {}", &self.props.background) }</span>
                <span>{ format!("Race: {}", &self.props.race) }</span>
                <span>{ format!("Alignment: {}", &self.props.alignment) }</span>
                <span>{ format!("Experience Points: {}", self.props.experience_points) }</span>
            </section>
        }
    }
}
