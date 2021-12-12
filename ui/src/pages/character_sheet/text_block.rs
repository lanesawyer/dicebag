use yew::{function_component, html, Html, Properties};

#[derive(Properties, Clone, PartialEq)]
pub struct TextBlockProps {
    pub name: String,
    pub value: Option<String>,
}

#[function_component(TextBlock)]
pub fn text_block(props: &TextBlockProps) -> Html {
    html! {
        <section class="text-block">
            <h3>{ &props.name }</h3>
            <div>{ &props.value.as_ref().unwrap_or(&"".to_string()) }</div>
        </section>
    }
}
