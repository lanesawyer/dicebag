use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

use crate::components::Icon;

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
                <Icon name="pulse" />
                <span>{ "0" }</span>
                <meter
                    min="0"
                    low=self.calc_low_hp()
                    max=self.props.maximum.to_string()
                    value=self.props.current.to_string()></meter>
                <span>{ self.props.maximum }</span>
                <div>{ format!("Current: {}", self.props.current) }</div>
                // TODO: Figure out extra bar to show temporary hit points
                <div>{ format!("Temporary: {}", self.props.temporary) }</div>
            </section>
        }
    }
}

impl HitPoints {
    fn calc_low_hp(&self) -> String {
        (self.props.maximum as f64 * 0.50).to_string()
    }
}
