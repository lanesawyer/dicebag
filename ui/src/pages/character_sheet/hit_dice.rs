use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

#[derive(Properties, Clone, PartialEq)]
pub struct HitDiceProps {
    pub total: i64,
    pub used: i64,
}

pub struct HitDice {
    pub props: HitDiceProps,
}

impl Component for HitDice {
    type Message = ();
    type Properties = HitDiceProps;

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
            <section id="hit-dice" class="text-block">
                <h3>{ "Hit Dice" }</h3>
                <div>{ self.props.total }</div>
                <div>{ self.props.used }</div>
            </section>
        }
    }
}
