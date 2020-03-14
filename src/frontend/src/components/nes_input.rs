use yew::{html, Properties, Component, ComponentLink, Html, ShouldRender};

pub struct NesInput {
    link: ComponentLink<Self>,
    props: Props,
}

#[derive(Clone)]
pub enum InputType {
	Text,
	Password,
}

impl Default for InputType {
	fn default () -> Self { InputType::Text }
}


#[derive(Clone, Properties)]
pub struct Props {
	#[prop_or_default]	
	pub label: Option<String>,
	#[prop_or_default]
	pub itype: InputType,
	pub identifier: String,
}


impl Component for NesInput {
    type Properties = Props;
    type Message = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        NesInput { link, props }
    }

    fn update(&mut self, _msg: Self::Message,) -> ShouldRender {
	    true
    }

    fn view(&self) -> Html {
	let maybe_display_label = move || -> Html {
		if self.props.label.is_some() {
			html! {
				<label for={&self.props.identifier}>{self.props.label.as_ref().unwrap()}</label>
			}
		} else {
			html! {}
		}
	};
	let itype = move || -> &str {
		match self.props.itype {
		InputType::Text => "text",
		InputType::Password => "password",
		}
	};

        html! {
		<>
		{ maybe_display_label() }
		<input class="nes-input" name={&self.props.identifier} type={itype()}/>
		</>
        }
    }
}
