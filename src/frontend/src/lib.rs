#![recursion_limit = "512"]

extern crate anyhow;
extern crate cfg_if;
extern crate serde;
extern crate wasm_bindgen;
extern crate wasm_bindgen_futures;
extern crate web_sys;
extern crate yew;
extern crate yew_router;

use wasm_bindgen::prelude::*;

pub mod components;
pub mod mite;
pub mod views;

// Called by our JS entry point
#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    yew::start_app::<views::index::Index>();
    Ok(())
}
