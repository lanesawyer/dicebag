#![recursion_limit = "1024"]

use yew::prelude::*;
use yew::{html, Component, Html};
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
    CharacterSheet {
        id: i64
    },
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

pub struct Dicebag;

impl Component for Dicebag {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let _callback = ctx.link().send_message(Msg::UpdateRoute);

        Self
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateRoute => true,
        }
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <BrowserRouter>
                { self.view_nav(ctx) }
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
}

impl Dicebag {
    fn view_nav(&self, _ctx: &Context<Self>) -> Html {
        // let route = ctx.link().location().expect("location was not available").pathname();
        let route = "/";
        html! {
            <nav>
                <h1>{ "🎲 Dicebag" }</h1>
                <ul>
                    <li>
                        <Link<AppRoute> classes={set_active_route(route, "/")} to={AppRoute::Home}>
                            <Icon name="home" />
                            { "Home" }
                        </Link<AppRoute>>
                    </li>
                    <li>
                        <Link<AppRoute> classes={set_active_route(route, "/characters")} to={AppRoute::Characters}>
                            <Icon name="people" />
                            { "Characters" }
                        </Link<AppRoute>>
                    </li>
                    <li>
                        <Link<AppRoute> classes={set_active_route(route, "/campaigns")} to={AppRoute::Campaigns}>
                            <Icon name="map" />
                            { "Campaigns" }
                        </Link<AppRoute>>
                    </li>
                </ul>
            </nav>
        }
    }
}

fn switch(routes: &AppRoute) -> Html {
    match routes {
        AppRoute::Home => html! { <HomePage /> },
        AppRoute::Characters => html!{ <CharactersPage /> },
        AppRoute::CharacterSheet { id } => html!{ <CharacterSheetPage id={*id} /> },
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
    yew::start_app::<Dicebag>();
}
