#![recursion_limit = "1024"]

use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_router::prelude::*;


mod character_sheet;
mod utils;
mod components;
mod dicebag;

use crate::dicebag::Dicebag;

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Dicebag>::new().mount_to_body();
}
