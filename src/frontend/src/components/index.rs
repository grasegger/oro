use yew::{html, Component, ComponentLink, Html, ShouldRender};
use web_sys::console;
use components::nes_container::NesContainer;
use components::nes_form::NesForm;
use components::nes_field::NesField;
use components::nes_input::{NesInput, InputType};
use components::nes_button::{NesButton, ButtonState};

pub struct Index {
    link: ComponentLink<Self>,
}

pub enum Msg {
	Login,
	Delete,
}


impl Component for Index {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Index { link }
    }

    fn update(&mut self, msg: Self::Message,) -> ShouldRender {
	    match msg {
		    Msg::Login => {
			    console::log_1(&"login".into());
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
