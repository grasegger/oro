use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

pub struct NesLinkButton {
    #[allow(dead_code)]
    link: ComponentLink<Self>,
    props: Props,
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

impl Default for ButtonState {
    fn default() -> Self {
        ButtonState::Empty
    }
}

#[derive(Clone, Properties)]
pub struct Props {
    #[prop_or_default]
    pub bstate: ButtonState,
    pub description: String,
    pub href: String,
}

impl NesLinkButton {
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

impl Component for NesLinkButton {
    type Properties = Props;
    type Message = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        NesLinkButton { link, props }
    }

    #[allow(non_snake_case)]
    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <a class=("nes-btn", self.get_state()) href={self.props.href.clone()}>{&self.props.description}</a>
        }
    }
}
