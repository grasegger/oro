use crate::views::login::Login;
use crate::views::projects::Projects;
use yew::{html, Component, ComponentLink, Html};
use yew_router::router::Router;
use yew_router::Switch;

#[derive(Switch, Debug, Clone)]
pub enum AppRoute {
    #[to = "/login"]
    Login,
    #[to = "/"]
    Index,
}

pub struct Index {
    #[allow(dead_code)]
    link: ComponentLink<Self>,
}

impl Component for Index {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Index { link: link }
    }

    fn update(&mut self, _: <Self as yew::Component>::Message) -> bool {
        false
    }

    fn view(&self) -> Html {
        html! {
            <Router<AppRoute, ()>
                render = Router::render(|switch: AppRoute| {
                    match switch {
                        AppRoute::Index => html!{<Projects/>},
                        AppRoute::Login => html!{<Login/>},
                    }
                })
            />
        }
    }
}
