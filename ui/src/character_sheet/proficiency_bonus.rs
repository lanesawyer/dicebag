use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

#[derive(Properties, Clone)]
pub struct ProficiencyBonus {
    pub value: usize,
}

impl Component for ProficiencyBonus {
    type Message = ();
    type Properties = ProficiencyBonus;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { value: props.value }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <section id="proficiency-bonus" class="single-value card">
                <div class="stat-value">{ self.value }</div>
                <div class="stat-name">{ "Proficiency Bonus" }</div>
            </section>
        }
    }
}
