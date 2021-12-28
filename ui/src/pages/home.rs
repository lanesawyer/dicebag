use yew::{function_component, html, Properties};

#[derive(Properties, Clone, PartialEq)]
pub struct HomePageProps;

#[function_component(HomePage)]
pub fn home_page(_props: &HomePageProps) -> Html {
    html! {
        <>
            { "Welcome to Dicebag!" }
        </>
    }
}
