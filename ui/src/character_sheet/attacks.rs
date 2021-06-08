use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

#[derive(Clone)]
pub struct Attack {
    pub name: String,
    pub bonus: usize,
    pub damage: String,
}

#[derive(Properties, Clone)]
pub struct Attacks {
    pub attacks: Vec<Attack>,
}

impl Component for Attacks {
    type Message = ();
    type Properties = Attacks;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {
            attacks: props.attacks,
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
            <section id="attacks-and-spellcasting" class="text-block">
                <h3>{"Attacks & Spellcasting"}</h3>
                <ul>
                { self.attacks.iter().map(|attack| html! {
                    <li>
                        <span>{&attack.name}</span>
                        <span>{attack.bonus}</span>
                        <span>{&attack.damage}</span>
                    </li>
                }).collect::<Html>() }
                </ul>
            </section>
        }
    }
}
