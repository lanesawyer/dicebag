use yew::{function_component, html};

#[function_component(NotFoundPage)]
pub fn not_found_page() -> Html {
    // TODO: Something clever, like perception check failed or the chest was empty
    html! {
        <>
            { "Not found" }
        </>
    }
}
