use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

#[derive(Properties, Clone, PartialEq)]
pub struct HitPointsProps {
    pub maximum: i64,
    pub current: i64,
    pub temporary: i64,
}

pub struct HitPoints {
    pub props: HitPointsProps,
}

impl Component for HitPoints {
    type Message = ();
    type Properties = HitPointsProps;

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
            <section id="hit-points" class="text-block">
                <h3>{ "Hit Points" }</h3>
                <div>{ format!("Maximum: {}", self.props.maximum) }</div>
                <div>{ format!("Current: {}", self.props.current) }</div>
                <div>{ format!("Temporary: {}", self.props.temporary) }</div>
            </section>
        }
    }
}
