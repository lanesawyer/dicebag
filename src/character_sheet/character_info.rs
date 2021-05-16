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
            <section class="text-block">
                <div>{ &self.name }</div>
                <div>{ &self.class }</div>
                <div>{ self.level }</div>
                <div>{ &self.background }</div>
                <div>{ &self.race }</div>
                <div>{ &self.alignment }</div>
                <div>{ self.experience_points }</div>
            </section>
        }
    }
}
