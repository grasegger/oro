use yew::{html, Component, ComponentLink, Html, ShouldRender, MouseEvent};
use web_sys::console;

pub struct Index {
    link: ComponentLink<Self>,
}

pub enum Msg {
    Click(MouseEvent),
}


impl Component for Index {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Index { link }
    }

    fn update(&mut self, msg: Self::Message,) -> ShouldRender {
	    match msg {
		    Msg::Click (MouseEvent) => {
			    MouseEvent.prevent_default();
			    console::log_1(&"clock".into());
		    }
	    }
	    true
    }

    fn view(&self) -> Html {
        html! {
            <div class="nes-container with-title">
		  <h3 class="title">{"Quest 1: Tell Me About Yourself"}</h3>
			<form>
			  <div class="nes-field">
			    <label for="instance">{"Mite Instance"}</label>
			    <input class="nes-input" name="instance"/>
			  </div>
		
			  <div class="nes-field">
			    <label for="apikey">{"Mite API Key"}</label>
			    <input class="nes-input" type="password" name="apikey"/>
			  </div>
			  
			  <div class="nes-field">
			    <input type="submit" class="nes-btn is-primary" value={"Complete Quest"} onclick=self.link.callback(|e| Msg::Click(e))/>
			  </div>
			</form>
            </div>
        }
    }
}
