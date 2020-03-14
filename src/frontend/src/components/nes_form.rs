use yew::{html, Renderable, Children, Properties, Component, ComponentLink, Html, ShouldRender};


pub struct NesForm {
    link: ComponentLink<Self>,
    props: Props,
}

#[derive(Clone, Properties)]
pub struct Props {
	pub children: Children,
}


impl Component for NesForm {
    type Properties = Props;
    type Message = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        NesForm { link, props }
    }

    fn update(&mut self, _msg: Self::Message,) -> ShouldRender {
	    true
    }

    fn view(&self) -> Html {
        html! {
		<form>
		{ self.props.children.render() }
		</form>
        }
    }
}
