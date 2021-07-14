use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

#[derive(Properties, Clone, PartialEq)]
pub struct AttackProps {
    pub attacks: Vec<Attack>,
}

#[derive(Clone, PartialEq)]
pub struct Attack {
    pub name: String,
    pub bonus: i64,
    pub damage: i64,
}

#[derive(Properties, Clone)]
pub struct Attacks {
    pub props: AttackProps,
}

impl Component for Attacks {
    type Message = ();
    type Properties = AttackProps;

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
            <section id="attacks-and-spellcasting" class="text-block">
                <h3>{"Attacks & Spellcasting"}</h3>
                <ul>
                { self.props.attacks.iter().map(|attack| html! {
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
