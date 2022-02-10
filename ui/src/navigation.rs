use yew::{function_component, html, Html};
use yew_router::history::Location;
use yew_router::hooks::use_location;
use yew_router::prelude::*;
use yew_router::Routable;

use crate::components::Icon;
use crate::pages::{
    CampaignPage, CampaignsPage, CharacterSheetPage, CharactersPage, HomePage, NotFoundPage,
};

// Matches from most specific to least
// so if you don't see the page, it's probably the wrong order
#[derive(Routable, PartialEq, Clone, Debug)]
pub enum AppRoute {
    #[at("/characters/:id")]
    CharacterSheet { id: i64 },
    #[at("/characters")]
    Characters,
    #[at("/campaigns/:id")]
    Campaign { id: i64 },
    #[at("/campaigns")]
    Campaigns,
    #[not_found]
    #[at("/404")]
    NotFound,
    // Needs to go last otherwise it will match everything
    #[at("/")]
    Home,
}

#[function_component(Navigation)]
pub fn navigation() -> Html {
    let location = use_location().expect("location should be available");
    let route = location.pathname();
    html! {
        <nav>
            <h1>{ "🎲 Dicebag" }</h1>
            <ul>
                <li>
                    <Link<AppRoute> classes={set_active_route(&route, "/")} to={AppRoute::Home}>
                        <Icon name="home" />
                        { "Home" }
                    </Link<AppRoute>>
                </li>
                <li>
                    <Link<AppRoute> classes={set_active_route(&route, "/characters")} to={AppRoute::Characters}>
                        <Icon name="people" />
                        { "Characters" }
                    </Link<AppRoute>>
                </li>
                <li>
                    <Link<AppRoute> classes={set_active_route(&route, "/campaigns")} to={AppRoute::Campaigns}>
                        <Icon name="map" />
                        { "Campaigns" }
                    </Link<AppRoute>>
                </li>
            </ul>
        </nav>
    }
}

pub fn switch(routes: &AppRoute) -> Html {
    match routes {
        AppRoute::Home => html! { <HomePage /> },
        AppRoute::Characters => html! { <CharactersPage /> },
        AppRoute::CharacterSheet { id } => html! { <CharacterSheetPage id={*id} /> },
        AppRoute::Campaigns => html! { <CampaignsPage /> },
        AppRoute::Campaign { id } => html! { <CampaignPage id={*id} /> },
        AppRoute::NotFound => html! { <NotFoundPage /> },
        // TODO: Unauthorized page, something like "you failed to pick the lock"
    }
}

fn set_active_route(current_route: &str, path: &'static str) -> &'static str {
    match path {
        "/" if current_route == path => "active",
        _ if path != "/" && current_route.starts_with(path) => "active",
        _ => "",
    }
}
