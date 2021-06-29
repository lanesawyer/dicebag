use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

#[derive(Properties, Clone, PartialEq)]
pub struct DeathSavingThrowsProps {
    pub saves: i64,
    pub failures: i64,
}

pub struct DeathSavingThrows {
    pub props: DeathSavingThrowsProps,
}

impl Component for DeathSavingThrows {
    type Message = ();
    type Properties = DeathSavingThrowsProps;

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
            <section id="death-saving-throws" class="text-block">
                <h3>{ "Death Saving Throws" }</h3>
                <h4>{ "Successes" }</h4>
                <input type="checkbox" checked={self.props.saves >= 1} />
                <input type="checkbox" checked={self.props.saves >= 2} />
                <input type="checkbox" checked={self.props.saves >= 3}/>

                <h4>{ "Failures" }</h4>
                <input type="checkbox" checked={self.props.failures >= 1} />
                <input type="checkbox" checked={self.props.failures >= 2} />
                <input type="checkbox" checked={self.props.failures >= 3} />
            </section>
        }
    }
}
