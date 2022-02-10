use yew::{function_component, html};

use crate::services::use_form::FormTest;

#[function_component(HomePage)]
pub fn home_page() -> Html {
    html! {
        <>
            { "Welcome to Dicebag!" }
            <FormTest />
        </>
    }
}
