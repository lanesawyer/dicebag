use yew::{function_component, html};

#[function_component(HomePage)]
pub fn home_page() -> Html {
    html! {
        <>
            { "Welcome to Dicebag!" }
        </>
    }
}
