use super::secure_view::SecureView;
use crate::components::oro_projectlist::ProjectList;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct Projects {
#[allow(dead_code)]
    link: ComponentLink<Self>,
}


impl SecureView for Projects {
    fn render_secured_view(&self) -> Html {
        html! {
            <>
            <ProjectList apikey=Projects::get_api_key() instance=Projects::get_instance()/>
            </>
        }
    }
}

impl Component for Projects {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Projects {
            link,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        self.secured_view()
    }
}
