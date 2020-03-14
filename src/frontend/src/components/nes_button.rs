use yew::{html, Callback, Properties, Component, ComponentLink, Html, ShouldRender, MouseEvent};

pub struct NesButton {
    link: ComponentLink<Self>,
    props: Props,
}

pub enum Msg {
    Clicked(MouseEvent),
}

#[derive(Clone)]
pub enum ButtonType {
	Submit,
	Reset,
	Button,
	Empty,
}

#[derive(Clone)]
pub enum ButtonState {
	Empty,
	Primary,
	Success,
	Warning,
	Error,
	Disabled,
}

impl Default for ButtonType {
	fn default () -> Self { ButtonType::Empty }
}

impl Default for ButtonState {
	fn default () -> Self { ButtonState::Empty }
}


#[derive(Clone, Properties)]
pub struct Props {
	#[prop_or_default]
	pub btype: ButtonType,
	#[prop_or_default]
	pub bstate: ButtonState,
	pub description: String,
	pub onsignal: Callback<()>,
}

impl NesButton {
	fn getType (&self) -> &str {
		match self.props.btype {
			ButtonType::Empty => "",
			ButtonType::Submit => "submit",
			ButtonType::Button => "button",
			ButtonType::Reset => "reset",
		}
	}

	fn getState (&self) -> &str {
		match self.props.bstate {
		ButtonState::Empty => "",
		ButtonState::Primary => "is-primary",
		ButtonState::Success => "is-success",
		ButtonState::Warning => "is-warning",
		ButtonState::Error => "is-error",
		ButtonState::Disabled => "is-disabled",
		}
	}
}

impl Component for NesButton {
    type Properties = Props;
    type Message = Msg;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        NesButton { link, props, }
    }

    fn update(&mut self, msg: Self::Message,) -> ShouldRender {
	 match msg {
            Msg::Clicked (MouseEvent) => {
		    MouseEvent.prevent_default();
		    self.props.onsignal.emit(());
            }
        }
	false
    }

    fn view(&self) -> Html {
        html! {
		<button type={self.getType()} onclick=self.link.callback(|e| Msg::Clicked(e)) class=("nes-btn", self.getState())>{&self.props.description}</button>
        }
    }
}
