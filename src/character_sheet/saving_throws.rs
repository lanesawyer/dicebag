use crate::utils::calculate_modifier;
use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

#[derive(Clone)]
pub struct SavingThrow {
    pub has_proficiency: bool,
    pub ability_score: usize,
    pub name: String,
}

#[derive(Properties, Clone)]
pub struct SavingThrows {
    pub items: Vec<SavingThrow>,
}

impl Component for SavingThrows {
    type Message = ();
    type Properties = SavingThrows;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { items: props.items }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <section id="saving-throws" class="text-block">
                <h3>{"Saving Throws"}</h3>
                <ul>
                { self.items.iter().map(|item| html! {
                    <li>
                        <input type="checkbox" checked=item.has_proficiency />
                        <span>{calculate_modifier(item.ability_score)}</span>
                        <span>{&item.name}</span>
                    </li>
                }).collect::<Html>() }
                </ul>
            </section>
        }
    }
}
