use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

#[derive(Properties, Clone)]
pub struct PassivePerception {
    pub value: i64,
}

impl Component for PassivePerception {
    type Message = ();
    type Properties = PassivePerception;

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
            <div id="passive-perception" class="single-value">
                <div class="stat-value">{ self.value }</div>
                <div class="stat-name">{ "Passive Perception" }</div>
            </div>
        }
    }
}