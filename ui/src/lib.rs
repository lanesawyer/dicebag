#![recursion_limit = "1024"]

use wasm_bindgen::prelude::*;
use yew::prelude::*;

mod character_sheet;
mod characters;
mod components;
mod dice_tower;
mod dicebag;
mod home;
mod services;
mod utils;

use crate::dicebag::Dicebag;

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Dicebag>::new().mount_to_body();
}
