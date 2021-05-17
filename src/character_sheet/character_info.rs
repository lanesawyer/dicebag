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
                <span>{ &self.name }</span>
                <span>{ &self.class }</span>
                <span>{ self.level }</span>
                <span>{ &self.background }</span>
                <span>{ &self.race }</span>
                <span>{ &self.alignment }</span>
                <span>{ self.experience_points }</span>
            </section>
        }
    }
}
