use std::num::NonZeroU8;
use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yew::services::ConsoleService;

use crate::dice_tower::Roll;

pub enum Msg {
    RollD4,
    RollD6,
    RollD8,
    RollD10,
    RollD12,
    RollD20,
    RollD100,
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

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let (number, dice) = match msg {
            Msg::RollD4 => (NonZeroU8::new(1).unwrap(), crate::dice_tower::DiceType::D4),
            Msg::RollD6 => (NonZeroU8::new(1).unwrap(), crate::dice_tower::DiceType::D6),
            Msg::RollD8 => (NonZeroU8::new(1).unwrap(), crate::dice_tower::DiceType::D8),
            Msg::RollD10 => (NonZeroU8::new(1).unwrap(), crate::dice_tower::DiceType::D10),
            Msg::RollD12 => (NonZeroU8::new(1).unwrap(), crate::dice_tower::DiceType::D12),
            Msg::RollD20 => (NonZeroU8::new(1).unwrap(), crate::dice_tower::DiceType::D20),
            Msg::RollD100 => (NonZeroU8::new(1).unwrap(), crate::dice_tower::DiceType::D100),
        };

        let result = Roll::roll(&Roll {
            number,
            dice
        });

        ConsoleService::log(&format!("{:?}", result));

        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let d4_click = self.link.callback(|_| Msg::RollD4);
        let d6_click = self.link.callback(|_| Msg::RollD6);
        let d8_click = self.link.callback(|_| Msg::RollD8);
        let d10_click = self.link.callback(|_| Msg::RollD10);
        let d12_click = self.link.callback(|_| Msg::RollD12);
        let d20_click = self.link.callback(|_| Msg::RollD20);
        let d100_click = self.link.callback(|_| Msg::RollD100);

        html! {
            <section id="dice-tower" class="text-block">
                <h3>{ "Dice Tower" }</h3>
                <button onclick=d4_click>{ "D4" }</button>
                <button onclick=d6_click>{ "D6" }</button>
                <button onclick=d8_click>{ "D8" }</button>
                <button onclick=d10_click>{ "D10" }</button>
                <button onclick=d12_click>{ "D12" }</button>
                <button onclick=d20_click>{ "D20" }</button>
                <button onclick=d100_click>{ "D100" }</button>
                <div>{ Roll::roll(&Roll {
                    number: NonZeroU8::new(1).unwrap(),
                    dice: crate::dice_tower::DiceType::D10,
                }) }</div>
            </section>
        }
    }
}
