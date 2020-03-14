use yew::{html, Renderable, Children, Properties, Component, ComponentLink, Html, ShouldRender};


pub struct NesField {
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

    fn update(&mut self, msg: Self::Message,) -> ShouldRender {
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
