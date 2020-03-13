
#![recursion_limit = "512"]

extern crate cfg_if;
extern crate wasm_bindgen;
extern crate web_sys;
extern crate yew; 

use wasm_bindgen::prelude::*;

mod components;

// Called by our JS entry point
#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    yew::start_app::<components::clock::Model>();
    Ok(())
}
