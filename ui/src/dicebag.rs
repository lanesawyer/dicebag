use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yew_router::{prelude::*, Switch};

use crate::{character_sheet::sheet::CharacterSheet, home::Home};

#[derive(Switch, PartialEq, Clone, Debug)]
pub enum Route {
    #[to = "/character"]
    CharacterSheet,
    // #[not_found] isn't in 0.18 but it's coming
    #[to = "/404"]
    NotFound,
    // Needs to go last otherwise it will match everything
    #[to = "/"]
    Home,
}

pub enum Msg {
    ToggleNavbar,
}

pub struct Dicebag {
    link: ComponentLink<Self>,
    navbar_active: bool,
}

impl Component for Dicebag {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            navbar_active: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ToggleNavbar => {
                self.navbar_active = !self.navbar_active;
                true
            }
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
        let Self {
            ref link,
            navbar_active,
            ..
        } = *self;

        let _active_class = if navbar_active { "is-active" } else { "" };

        html! {
            <nav class="navbar is-primary" role="navigation" aria-label="main navigation">
                <div class="navbar-brand">
                    <h1 class="navbar-item is-size-3">{ "Dicebag" }</h1>

                    <a role="button"
                        class={"navbar-burger burger"}
                        aria-label="menu" aria-expanded="false"
                        onclick=link.callback(|_| Msg::ToggleNavbar)
                    >
                        <span aria-hidden="true"></span>
                        <span aria-hidden="true"></span>
                        <span aria-hidden="true"></span>
                    </a>
                </div>
                <ul>
                    <li>
                        <RouterAnchor<Route> classes={"navbar-item"} route=Route::Home>
                            { "Home" }
                        </RouterAnchor<Route>>
                    </li>
                    <li>
                        <RouterAnchor<Route> classes={"navbar-item"} route=Route::CharacterSheet>
                            { "Character Sheet" }
                        </RouterAnchor<Route>>
                    </li>
                </ul>
            </nav>
        }
    }
}

fn routes(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::CharacterSheet => html! { <CharacterSheet /> },
        Route::NotFound => html! { <>{ "NOT FOUND" }</> },
    }
}
