use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

#[derive(Properties, Clone)]
pub struct HitDice {
    pub total: usize,
    pub used: usize,
}

impl Component for HitDice {
    type Message = ();
    type Properties = HitDice;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {
            total: props.total,
            used: props.used,
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
            <section id="hit-dice" class="text-block">
                <h2>{ "Hit Dice" }</h2>
                <div>{ self.total }</div>
                <div>{ self.used }</div>
            </section>
        }
    }
}
