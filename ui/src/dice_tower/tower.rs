use std::num::NonZeroU8;
use yew::{html, Component, Context, Html};

use core::{DiceType, Roll};

pub struct Tower {
    dice: DiceType,
}

impl Component for Tower {
    type Message = DiceType;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { dice: DiceType::D4 }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        let (_number, dice) = match msg {
            DiceType::D4 => (NonZeroU8::new(1).unwrap(), DiceType::D4),
            DiceType::D6 => (NonZeroU8::new(1).unwrap(), DiceType::D6),
            DiceType::D8 => (NonZeroU8::new(1).unwrap(), DiceType::D8),
            DiceType::D10 => (NonZeroU8::new(1).unwrap(), DiceType::D10),
            DiceType::D12 => (NonZeroU8::new(1).unwrap(), DiceType::D12),
            DiceType::D20 => (NonZeroU8::new(1).unwrap(), DiceType::D20),
            DiceType::D100 => (
                NonZeroU8::new(1).unwrap(),
                DiceType::D100,
            ),
            // Other dice types are not supported
            _ => return false,
        };

        self.dice = dice;

        true
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let d4_click = ctx.link().callback(|_| DiceType::D4);
        let d6_click = ctx.link().callback(|_| DiceType::D6);
        let d8_click = ctx.link().callback(|_| DiceType::D8);
        let d10_click = ctx.link().callback(|_| DiceType::D10);
        let d12_click = ctx.link().callback(|_| DiceType::D12);
        let d20_click = ctx.link().callback(|_| DiceType::D20);
        let d100_click = ctx.link().callback(|_| DiceType::D100);

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
                <div>{ core::Roll::roll(&Roll {
                    number: NonZeroU8::new(1).unwrap(),
                    dice: self.dice,
                }) }</div>
            </section>
        }
    }
}
