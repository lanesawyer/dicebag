use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yew_router::{prelude::*, service::RouteService, Switch};

use crate::{character_sheet::sheet::CharacterSheet, characters::CharactersPage, home::Home};

// Matches from most specific to least
// so if you don't see the page, it's probably the wrong order
#[derive(Switch, PartialEq, Clone, Debug)]
pub enum Route {
    #[to = "/characters/{id}"]
    CharacterSheet(i32),
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
                    <Router<Route, ()> render = Router::render(routes) />
                </main>
                <footer>
                    { "Powered by " }
                    <a href="https://yew.rs">{ "Yew" }</a>
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
                        <RouterAnchor<Route> classes={set_active_route(&route, "/")} route=Route::Home>
                            { "🏠 Home" }
                        </RouterAnchor<Route>>
                    </li>
                    <li>
                        <RouterAnchor<Route> classes={set_active_route(&route, "/characters")} route=Route::Characters>
                            { "⚔️ Characters" }
                        </RouterAnchor<Route>>
                    </li>
                    <li>
                        <RouterAnchor<Route> classes={set_active_route(&route, "/campaigns")} route=Route::Campaigns>
                            { "🗺️ Campaigns" }
                        </RouterAnchor<Route>>
                    </li>
                </ul>
            </nav>
        }
    }
}

fn routes(route: Route) -> Html {
    match route {
        Route::Home => html! { <Home /> },
        Route::Characters => html! { <CharactersPage /> },
        Route::CharacterSheet(id) => html! { <CharacterSheet id=id /> },
        Route::Campaigns => html! { <>{ "Campaigns" }</> },
        Route::NotFound => html! { <>{ "NOT FOUND" }</> },
    }
}

fn set_active_route(route: &str, path: &'static str) -> &'static str {
    match path {
        "/" if route == path => "active",
        _ if path != "/" && route.starts_with(path) => "active",
        _ => "",
    }
}
