#![recursion_limit = "1024"]

use wasm_bindgen::prelude::*;
use yew::prelude::*;

use character_sheet::character_sheet::CharacterSheet;

mod character_sheet;
mod utils;

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<CharacterSheet>::new().mount_to_body();
}
