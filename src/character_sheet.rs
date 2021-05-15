use yew::{html, Component, ComponentLink, Html, ShouldRender};

use crate::stat_block::StatBlock;

pub struct CharacterSheet {
    link: ComponentLink<Self>,
    value: i64,
}

pub enum Msg {
    AddOne,
}

impl Component for CharacterSheet {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, value: 0 }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => self.value += 1,
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <button onclick=self.link.callback(|_| Msg::AddOne)>{ "+1" }</button>
                <p>{ self.value }</p>

                <div class="stats">
                    <StatBlock name="Strength" value={12} />
                    <StatBlock name="Dexterity" value={8} />
                    <StatBlock name="Constitution" value={14} />
                    <StatBlock name="Intelligence" value={11} />
                    <StatBlock name="Wisdom" value={10} />
                    <StatBlock name="Charisma" value={9} />
                </div>
            </div>
        }
    }
}
