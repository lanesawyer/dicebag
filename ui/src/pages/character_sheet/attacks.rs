use yew::{function_component, html, Html, Properties};

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

#[function_component(Attacks)]
pub fn attacks(props: &AttackProps) -> Html {
    html! {
        <section id="attacks-and-spellcasting" class="text-block">
            <h3>{"Attacks & Spellcasting"}</h3>
            <ul>
            { props.attacks.iter().map(|attack| html! {
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
