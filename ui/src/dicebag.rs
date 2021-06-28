use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yew_router::{prelude::*, Switch};

use crate::{character_sheet::sheet::CharacterSheet, home::Home};

#[derive(Switch, PartialEq, Clone, Debug)]
pub enum Route {
    #[to = "/characters"]
    Characters,
    #[to = "/characters/{id}"]
    CharacterSheet(i32),
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
            <nav>
                <h1>{ "Dicebag" }</h1>
                <ul>
                    <li>
                        <RouterAnchor<Route> route=Route::Home>
                            { "Home" }
                        </RouterAnchor<Route>>
                    </li>
                    <li>
                        <RouterAnchor<Route> route=Route::Characters>
                            { "Characters" }
                        </RouterAnchor<Route>>
                    </li>
                    <li>
                        <RouterAnchor<Route> route=Route::Campaigns>
                            { "Campaigns" }
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
        Route::Characters => html! { <CharacterSheet id={0} /> },
        Route::CharacterSheet(id) => html! { <CharacterSheet id={id} /> },
        Route::Campaigns => html! { <>{ "Campaigns" }</> },
        Route::NotFound => html! { <>{ "NOT FOUND" }</> },
    }
}
