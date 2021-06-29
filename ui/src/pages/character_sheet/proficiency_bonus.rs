use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

#[derive(Properties, Clone, PartialEq)]
pub struct ProficiencyBonusProps {
    pub value: i64,
}

pub struct ProficiencyBonus {
    pub props: ProficiencyBonusProps,
}

impl Component for ProficiencyBonus {
    type Message = ();
    type Properties = ProficiencyBonusProps;

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
            <section id="proficiency-bonus" class="single-value card">
                <div class="stat-value">{ self.props.value }</div>
                <div class="stat-name">{ "Proficiency Bonus" }</div>
            </section>
        }
    }
}
