use yew::{html, Callback, Component, ComponentLink, Html, MouseEvent, Properties, ShouldRender};

pub struct NesButton {
    #[allow(dead_code)]
    link: ComponentLink<Self>,
    props: Props,
}

pub enum Msg {
    Clicked(MouseEvent),
}

#[derive(Clone)]
#[allow(dead_code)]
pub enum ButtonType {
    Submit,
    Reset,
    Button,
    Empty,
}

#[derive(Clone)]
#[allow(dead_code)]
pub enum ButtonState {
    Empty,
    Primary,
    Success,
    Warning,
    Error,
    Disabled,
}

impl Default for ButtonType {
    fn default() -> Self {
        ButtonType::Empty
    }
}

impl Default for ButtonState {
    fn default() -> Self {
        ButtonState::Empty
    }
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
    fn get_type(&self) -> &str {
        match self.props.btype {
            ButtonType::Empty => "",
            ButtonType::Submit => "submit",
            ButtonType::Button => "button",
            ButtonType::Reset => "reset",
        }
    }

    fn get_state(&self) -> &str {
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
        NesButton { link, props }
    }

    #[allow(non_snake_case)]
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Clicked(MouseEvent) => {
                MouseEvent.prevent_default();
                self.props.onsignal.emit(());
            }
        }
        false
    }

    fn view(&self) -> Html {
        html! {
        <button type={self.get_type()} onclick=self.link.callback(|e| Msg::Clicked(e)) class=("nes-btn", self.get_state())>{&self.props.description}</button>
        }
    }
}
