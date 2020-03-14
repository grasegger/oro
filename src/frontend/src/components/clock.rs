use yew::{html, Component, ComponentLink, Html, ShouldRender};
use web_sys::console;

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
            Msg::Click => {
				console::log_1(&"clock".into())
			}
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