use yew::{html, Children, Component, ComponentLink, Html, Properties, Renderable, ShouldRender};

pub struct NesContainer {
    #[allow(dead_code)]
    link: ComponentLink<Self>,
    props: Props,
}

#[derive(Clone, Properties)]
pub struct Props {
    #[prop_or_default]
    pub title: Option<String>,
    #[prop_or(false)]
    pub centered: bool,
    #[prop_or(false)]
    pub dark: bool,
    pub children: Children,
}

impl Component for NesContainer {
    type Properties = Props;
    type Message = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        NesContainer { link, props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let mut classes = "nes-container".into();
        if self.props.title.is_some() {
            classes = format!("{} {}", classes, "with-title")
        }

        if self.props.dark {
            classes = format!("{} {}", classes, "is-dark")
        }

        if self.props.centered {
            classes = format!("{} {}", classes, "is-centered")
        }

        let maybe_render_label = move || -> Html {
            if self.props.title.is_some() {
                html! {
                    <p class="title">{self.props.title.as_ref().unwrap()}</p>
                }
            } else {
                html! {}
            }
        };

        html! {
        <div class={classes}>
        { maybe_render_label () }
        { self.props.children.render() }
        </div>
        }
    }
}
