use std::num::NonZeroU8;

use yew::{html, Component, ComponentLink, Html, ShouldRender};

use crate::dice_tower::Roll;

pub enum Msg {
    RollD10,
}

pub struct Tower {
    link: ComponentLink<Self>,
}

impl Component for Tower {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let d10_click = self.link.callback(|_| Msg::RollD10);
        html! {
            <section id="dice-tower" class="text-block">
                <h3>{ "Dice Tower" }</h3>
                <button onclick=d10_click>{ "Roll 1 D10" }</button>
                <div>{ Roll::roll(&Roll {
                    number: NonZeroU8::new(1).unwrap(),
                    dice: crate::dice_tower::DiceType::D10,
                }) }</div>
            </section>
        }
    }
}