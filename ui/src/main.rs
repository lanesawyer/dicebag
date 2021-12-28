#![recursion_limit = "1024"]

use yew::prelude::*;
use yew::{html, Html};
use yew_router::prelude::*;

use crate::components::Icon;
use crate::dice_tower::tower::Tower;
use crate::pages::{CharacterSheetPage, CharactersPage, HomePage};

mod components;
mod dice_tower;
mod pages;
mod services;
mod utils;

// Matches from most specific to least
// so if you don't see the page, it's probably the wrong order
#[derive(Routable, PartialEq, Clone, Debug)]
pub enum AppRoute {
    #[at("/characters/:id")]
    CharacterSheet { id: i64 },
    #[at("/characters")]
    Characters,
    #[at("/campaigns")]
    Campaigns,
    #[not_found]
    #[at("/404")]
    NotFound,
    // Needs to go last otherwise it will match everything
    #[at("/")]
    Home,
}

pub enum Msg {
    UpdateRoute,
}

#[derive(Properties, Clone, PartialEq, Default)]
pub struct DicebagProps;

#[function_component(Dicebag)]
pub fn dicebag(_props: &DicebagProps) -> Html {
    html! {
        <BrowserRouter>
            <ViewNav />
            <main>
                <Switch<AppRoute> render={Switch::render(switch)} />
            </main>
            <footer>
                <Tower />
                <a href="https://yew.rs">
                    <img src="/assets/yew-logo.png" alt="yew logo" />
                </a>
                <a href="https://github.com/lanesawyer/dicebag">
                    <img src="/assets/github-logo.png" alt="github logo" />
                </a>
            </footer>
        </BrowserRouter>
    }
}

#[derive(Properties, PartialEq, Clone, Debug)]
pub struct ViewNavProps;

#[function_component(ViewNav)]
pub fn view_nav(_props: &ViewNavProps) -> Html {
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

fn switch(routes: &AppRoute) -> Html {
    match routes {
        AppRoute::Home => html! { <HomePage /> },
        AppRoute::Characters => html! { <CharactersPage /> },
        AppRoute::CharacterSheet { id } => html! { <CharacterSheetPage id={*id} /> },
        AppRoute::Campaigns => html! { <>{ "Campaigns" }</> },
        AppRoute::NotFound => html! { <>{ "NOT FOUND" }</> },
    }
}

fn set_active_route(current_route: &str, path: &'static str) -> &'static str {
    match path {
        "/" if current_route == path => "active",
        _ if path != "/" && current_route.starts_with(path) => "active",
        _ => "",
    }
}

pub fn main() {
    yew::start_app::<Dicebag>();
}
