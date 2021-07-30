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
            <section id="proficiency-bonus" class="text-block">
                <h3>{ "Proficiency Bonus" }</h3>
                <div>{ self.props.value }</div>
            </section>
        }
    }
}
