use yew::prelude::*;
use yew::{html, Html};

pub struct Icon {
    props: IconProps,
}

#[derive(Properties, Clone, PartialEq)]
pub struct IconProps {
    pub name: String,
}

impl Component for Icon {
    type Message = ();
    type Properties = IconProps;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
            <img src=format!("/assets/icons/{}.svg", self.props.name) alt=self.props.name.clone() class="icon" />
        }
    }
}
