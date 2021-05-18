use yew::{html, Component, ComponentLink, Html, ShouldRender};

use crate::character_sheet::character_sheet::CharacterSheet;

// TODO: The Router just got an update and it hasn't shipped with any versions of Yew
// yet but this is the basic setup for when it does
// #[derive(Routable, PartialEq, Clone, Debug)]
// pub enum Route {
//     #[at("/")]
//     Home,
//     #[at("/character")]
//     CharacterSheet,
//     #[not_found]
//     #[at("/404")]
//     NotFound,
// }

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
                    // <Router<Route> render=Router::render(switch) />
                    <CharacterSheet />
                </main>
                <footer class="footer">
                    <div class="content has-text-centered">
                        { "Powered by " }
                        <a href="https://yew.rs">{ "Yew" }</a>
                    </div>
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

        let active_class = if navbar_active { "is-active" } else { "" };

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
                <div class={"navbar-menu"}>
                    <div class="navbar-start">
                        // <Link<Route> classes={"navbar-item"} route=Route::Home>
                        //     { "Home" }
                        // </Link<Route>>
                        // <Link<Route> classes={"navbar-item"} route=Route::CharacterSheet>
                        //     { "Character Sheet" }
                        // </Link<Route>>
                    </div>
                </div>
            </nav>
        }
    }
}

// fn switch(routes: &Route) -> Html {
//     match routes {
//         Route::Home => {
//             html! { <Dicebag /> }
//         }
//         Route::CharacterSheet => {
//             html! { <CharacterSheet /> }
//         }
//         Route::NotFound => {
//             html! { <>{ "NOT FOUND" }</> }
//         }
//     }
// }