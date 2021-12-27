use std::num::NonZeroU8;
use gloo_console::log;
use yew::{html, Component, Html, Context};

use crate::dice_tower::Roll;

use super::DiceType;

pub enum RollMsg {
    D4,
    D6,
    D8,
    D10,
    D12,
    D20,
    D100,
}

pub struct Tower {
    dice: DiceType,
}

impl Component for Tower {
    type Message = RollMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            dice: DiceType::D4,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        let (number, dice) = match msg {
            RollMsg::D4 => (NonZeroU8::new(1).unwrap(), crate::dice_tower::DiceType::D4),
            RollMsg::D6 => (NonZeroU8::new(1).unwrap(), crate::dice_tower::DiceType::D6),
            RollMsg::D8 => (NonZeroU8::new(1).unwrap(), crate::dice_tower::DiceType::D8),
            RollMsg::D10 => (NonZeroU8::new(1).unwrap(), crate::dice_tower::DiceType::D10),
            RollMsg::D12 => (NonZeroU8::new(1).unwrap(), crate::dice_tower::DiceType::D12),
            RollMsg::D20 => (NonZeroU8::new(1).unwrap(), crate::dice_tower::DiceType::D20),
            RollMsg::D100 => (
                NonZeroU8::new(1).unwrap(),
                crate::dice_tower::DiceType::D100,
            ),
        };

        let result = Roll::roll(&Roll { number, dice });

        log!(&format!("{:?}", result));

        self.dice = dice;

        true
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let d4_click = ctx.link().callback(|_| RollMsg::D4);
        let d6_click = ctx.link().callback(|_| RollMsg::D6);
        let d8_click = ctx.link().callback(|_| RollMsg::D8);
        let d10_click = ctx.link().callback(|_| RollMsg::D10);
        let d12_click = ctx.link().callback(|_| RollMsg::D12);
        let d20_click = ctx.link().callback(|_| RollMsg::D20);
        let d100_click = ctx.link().callback(|_| RollMsg::D100);

        html! {
            <section id="dice-tower" class="text-block">
                <h3>{ "Dice Tower" }</h3>
                <button onclick={d4_click}>{ "D4" }</button>
                <button onclick={d6_click}>{ "D6" }</button>
                <button onclick={d8_click}>{ "D8" }</button>
                <button onclick={d10_click}>{ "D10" }</button>
                <button onclick={d12_click}>{ "D12" }</button>
                <button onclick={d20_click}>{ "D20" }</button>
                <button onclick={d100_click}>{ "D100" }</button>
                <div>{ Roll::roll(&Roll {
                    number: NonZeroU8::new(1).unwrap(),
                    dice: self.dice,
                }) }</div>
            </section>
        }
    }
}
