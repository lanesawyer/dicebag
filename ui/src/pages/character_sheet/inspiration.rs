use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

#[derive(Properties, Clone, PartialEq)]
pub struct InspirationProps {
    pub value: bool,
}

pub struct Inspiration {
    pub props: InspirationProps,
}

impl Component for Inspiration {
    type Message = ();
    type Properties = InspirationProps;

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
            <section id="inspiration" class="single-value card">
                <input type="checkbox" class="stat-value" checked=self.props.value />
                <div>{ "Inspiration" }</div>
            </section>
        }
    }
}
