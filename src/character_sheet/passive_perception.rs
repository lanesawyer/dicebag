use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

#[derive(Properties, Clone)]
pub struct PassivePerception {
    pub name: String,
    pub value: usize,
}

impl Component for PassivePerception {
    type Message = ();
    type Properties = PassivePerception;

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
            </div>
        }
    }
}
