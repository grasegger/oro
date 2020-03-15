use super::secure_view::SecureView;
use crate::components::nes_button::NesButton;
use web_sys::console;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct Model {
    link: ComponentLink<Self>,
}

pub enum Msg {
    Click,
}

impl SecureView for Model {
    fn render_secured_view(&self) -> Html {
        html! {
            <div>
                <NesButton onsignal=self.link.callback(|_| Msg::Click) description={"Clock"} />
            </div>
        }
    }
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Model { link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Click => console::log_1(&"clock".into()),
        }
        true
    }

    fn view(&self) -> Html {
        self.secured_view()
    }
}
