use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

#[derive(Properties, Clone, PartialEq)]
pub struct TextBlockProps {
    pub name: String,
    pub value: String,
}

pub struct TextBlock {
    pub props: TextBlockProps,
}

impl Component for TextBlock {
    type Message = ();
    type Properties = TextBlockProps;

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
            <section class="text-block">
                <h3>{ &self.props.name }</h3>
                <div>{ &self.props.value }</div>
            </section>
        }
    }
}
