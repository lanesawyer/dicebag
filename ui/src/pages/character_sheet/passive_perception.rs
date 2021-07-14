use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

#[derive(Properties, Clone, PartialEq)]
pub struct PassivePerceptionProps {
    pub value: i64,
}

pub struct PassivePerception {
    pub props: PassivePerceptionProps,
}

impl Component for PassivePerception {
    type Message = ();
    type Properties = PassivePerceptionProps;

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
            <div id="passive-perception" class="single-value">
                <div class="stat-value">{ self.props.value }</div>
                <div class="stat-name">{ "Passive Perception" }</div>
            </div>
        }
    }
}
