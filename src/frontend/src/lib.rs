#![recursion_limit = "512"]

extern crate cfg_if;
extern crate wasm_bindgen;
extern crate wasm_bindgen_futures;
extern crate web_sys;
extern crate yew;
extern crate yew_router;
extern crate serde;
extern crate anyhow;

use wasm_bindgen::prelude::*;

pub mod components;
pub mod views;
pub mod mite;

// Called by our JS entry point
#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    yew::start_app::<views::index::Index>();
    Ok(())
}
