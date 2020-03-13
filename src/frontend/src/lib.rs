
#![recursion_limit = "512"]

extern crate cfg_if;
extern crate wasm_bindgen;
extern crate web_sys;
extern crate yew; 

use wasm_bindgen::prelude::*;
use yew::{html, Callback, Component, ComponentLink, Html, ShouldRender};


// Called by our JS entry point
#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    yew::start_app::<Model>();
    Ok(())
}

pub struct Model {
    link: ComponentLink<Self>,
}

pub enum Msg {
    Click,
}


impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Model { link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Click => {}
        }
        true
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <button onclick=self.link.callback(|_| Msg::Click)>{ "Clock" }</button>
            </div>
        }
    }
}