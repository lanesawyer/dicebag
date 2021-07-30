#![recursion_limit = "1024"]

use yew::prelude::*;
use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yew_router::{prelude::*, service::RouteService, Switch};

use crate::dice_tower::tower::Tower;
use crate::pages::{CharacterSheetPage, CharactersPage, HomePage};

mod components;
mod dice_tower;
mod pages;
mod services;
mod utils;

// Matches from most specific to least
// so if you don't see the page, it's probably the wrong order
#[derive(Switch, PartialEq, Clone, Debug)]
pub enum AppRoute {
    #[to = "/characters/{id}"]
    CharacterSheet(i64),
    #[to = "/characters"]
    Characters,
    #[to = "/campaigns"]
    Campaigns,
    // #[not_found] isn't in 0.18 but it's coming
    #[to = "/404"]
    NotFound,
    // Needs to go last otherwise it will match everything
    #[to = "/"]
    Home,
}

pub enum Msg {
    UpdateRoute,
}

pub struct Dicebag {
    _route_agent: RouteAgentBridge,
    _link: ComponentLink<Self>,
}

impl Component for Dicebag {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(|_| Msg::UpdateRoute);
        let route_agent = RouteAgentBridge::new(callback);

        Self {
            _route_agent: route_agent,
            _link: link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateRoute => true,
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                { self.view_nav() }
                <main>
                    <Router<AppRoute, ()> render = Router::render(routes) />
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
            </>
        }
    }
}

impl Dicebag {
    fn view_nav(&self) -> Html {
        let route = RouteService::<()>::new().get_path();

        html! {
            <nav>
                <h1>{ "🎲 Dicebag" }</h1>
                <ul>
                    <li>
                        <RouterAnchor<AppRoute> classes={set_active_route(&route, "/")} route=AppRoute::Home>
                            { "🏠 Home" }
                        </RouterAnchor<AppRoute>>
                    </li>
                    <li>
                        <RouterAnchor<AppRoute> classes={set_active_route(&route, "/characters")} route=AppRoute::Characters>
                            { "⚔️ Characters" }
                        </RouterAnchor<AppRoute>>
                    </li>
                    <li>
                        <RouterAnchor<AppRoute> classes={set_active_route(&route, "/campaigns")} route=AppRoute::Campaigns>
                            { "🗺️ Campaigns" }
                        </RouterAnchor<AppRoute>>
                    </li>
                </ul>
            </nav>
        }
    }
}

fn routes(route: AppRoute) -> Html {
    match route {
        AppRoute::Home => html! { <HomePage /> },
        AppRoute::Characters => html! { <CharactersPage /> },
        AppRoute::CharacterSheet(id) => html! { <CharacterSheetPage id=id /> },
        AppRoute::Campaigns => html! { <>{ "Campaigns" }</> },
        AppRoute::NotFound => html! { <>{ "NOT FOUND" }</> },
    }
}

fn set_active_route(route: &str, path: &'static str) -> &'static str {
    match path {
        "/" if route == path => "active",
        _ if path != "/" && route.starts_with(path) => "active",
        _ => "",
    }
}

pub fn main() {
    App::<Dicebag>::new().mount_to_body();
}
