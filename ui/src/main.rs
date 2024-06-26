#![recursion_limit = "1024"]

use yew::html;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::dice_tower::tower::Tower;
use crate::navigation::{switch, AppRoute, Navigation};

mod components;
mod dice_tower;
mod navigation;
mod pages;
mod services;
mod utils;

#[function_component(Dicebag)]
pub fn dicebag() -> Html {
    #[allow(clippy::let_unit_value)]
    let navigation = html! { <Navigation /> };
    #[allow(clippy::let_unit_value)]
    let tower = html! { <Tower /> };
    html! {
        <BrowserRouter>
            { navigation }
            <main>
                <Switch<AppRoute> render={Switch::render(switch)} />
            </main>
            <footer>
                { tower }
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

pub fn main() {
    yew::start_app::<Dicebag>();
}
