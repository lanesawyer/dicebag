use crate::utils::calculate_modifier_display;
use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

#[derive(Properties, Clone)]
pub struct ProficiencyBonus {
    pub name: String,
    pub value: usize,
}

impl Component for ProficiencyBonus {
    type Message = ();
    type Properties = ProficiencyBonus;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {
            name: props.name,
            value: props.value,
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
            <div class="stat-block">
                <div class="stat-name">{ &self.name }</div>
                <div class="stat-value">{ self.value }</div>
                <div class="stat-modifier">{ calculate_modifier_display(self.value) }</div>
            </div>
        }
    }
}
