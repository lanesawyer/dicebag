use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

#[derive(Properties, Clone)]
pub struct HitDice {
    pub total: i64,
    pub used: i64,
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
                <h3>{ "Hit Dice" }</h3>
                <div>{ self.total }</div>
                <div>{ self.used }</div>
            </section>
        }
    }
}
