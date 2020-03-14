use yew::{html, Component, ComponentLink, Html, ShouldRender, NodeRef};
use web_sys::{console, HtmlInputElement};
use components::nes_container::NesContainer;
use components::nes_form::NesForm;
use components::nes_field::NesField;
use components::nes_input::{NesInput, InputType};
use components::nes_button::{NesButton, ButtonState};
use wasm_bindgen::JsCast;


pub struct Index {
    link: ComponentLink<Self>,
    node_ref: NodeRef,
}

pub enum Msg {
	Login,
	Delete,
}


impl Index {
	fn store_credentials(&self) {
	    let window = web_sys::window().expect("no global `window` exists");
	    let document = window.document().expect("should have a document on window");
		
		// todo: use a node ref instead
		let name_input = document.get_elements_by_name("instance").get(0).expect("How did you break this???");
		let instance = name_input.dyn_into::<HtmlInputElement>().unwrap().value();
		
		console::log_1(&format!("{:?}", instance).into());
	}
}

impl Component for Index {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Index { link: link, node_ref: NodeRef::default()}
    }

    fn update(&mut self, msg: Self::Message,) -> ShouldRender {
	    match msg {
		    Msg::Login => {
			    console::log_1(&"login".into());
				self.store_credentials();
		    },
		    Msg::Delete => {
			    console::log_1(&"delete".into());
		    }
	    }
	    true
    }

    fn view(&self) -> Html {
        html! {
		<NesContainer title="Create Your Character">
			<NesForm>
				<NesField>
				<NesInput identifier="instance" label="What mite realm do you serve for?" itype=InputType::Text />
				</NesField>

				<NesField>
				<NesInput identifier="apikey" label="How may you be identified?" itype=InputType::Password />
				</NesField>
				
				<NesField>
					<NesButton description="Start Adventure" bstate=ButtonState::Primary onsignal=self.link.callback(|_| Msg::Login) />
				</NesField>

				<NesField>
					<NesButton description="Delete Save File" bstate=ButtonState::Warning onsignal=self.link.callback(|_| Msg::Delete) />
				</NesField>
			</NesForm>
		</NesContainer>
	}
    }
}
