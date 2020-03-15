use yew::{html, Html, Component, ComponentLink};
use yew_router::{ Switch};
use yew_router::router::Router;
use components::login::Login;
use components::clock::Model;

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
                        AppRoute::Index => html!{<Model/>},
                        AppRoute::Login => html!{<Login/>},
                    }
                })
            />
        }
    }
}
