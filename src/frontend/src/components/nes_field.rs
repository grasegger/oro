use yew::{html, Children, Component, ComponentLink, Html, Properties, Renderable, ShouldRender};

pub struct NesField {
    #[allow(dead_code)]
    link: ComponentLink<Self>,
    props: Props,
}

#[derive(Clone, Properties)]
pub struct Props {
    pub children: Children,
}

impl Component for NesField {
    type Properties = Props;
    type Message = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        NesField { link, props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
        <div class="nes-field">
        { self.props.children.render() }
        </div>
        }
    }
}
