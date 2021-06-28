use crate::utils::calculate_modifier_display;
use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

#[derive(Properties, Clone, PartialEq)]
pub struct StatBlockProps {
    pub name: String,
    pub value: i64,
}

pub struct StatBlock {
    props: StatBlockProps,
}

impl Component for StatBlock {
    type Message = ();
    type Properties = StatBlockProps;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
        // if self.props != props {
        //     self.props = props;
        //     true
        // } else {
        //     false
        // }
    }

    fn view(&self) -> Html {
        html! {
            <div class="stat-block">
                <div class="stat-name">{ &self.props.name }</div>
                <div class="stat-value">{ self.props.value }</div>
                <div class="stat-modifier">{ calculate_modifier_display(self.props.value) }</div>
            </div>
        }
    }
}
