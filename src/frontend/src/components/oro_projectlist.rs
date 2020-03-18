use crate::components::nes_container::NesContainer;
use crate::components::nes_field::NesField;
use crate::components::nes_link_button::ButtonState;
use crate::components::nes_link_button::NesLinkButton;
use crate::mite::support_projects::MiteProject;
use wasm_bindgen::JsValue;
use web_sys::console;
use yew::format::Json;
use yew::services::fetch::FetchService;
use yew::services::fetch::FetchTask;
use yew::services::fetch::Response;
use yew::{html, Properties, Component, ComponentLink, Html, ShouldRender};

pub struct ProjectList {
#[allow(dead_code)]
    link: ComponentLink<Self>,
    _fetch_task: Option<FetchTask>,
    projects: Vec<MiteProject>,
    pub props: Props,
}

#[derive(Clone, Properties)]
pub struct Props {
    pub apikey: String,
    pub instance: String
}

pub enum Msg {
    ProjectsLoaded(Vec<MiteProject>),
}

impl ProjectList {
    fn load_data(link: ComponentLink<Self>, instance: String, apikey: String) -> Option<FetchTask> {
        let request =
            MiteProject::get_support_projects(instance, apikey);
        Some(
            FetchService::new()
                .fetch(
                    request,
                    (move |response: Response<Json<anyhow::Result<Vec<MiteProject>>>>| {
                        match response.into_body().0 {
                            Ok(data) => link.send_message(Msg::ProjectsLoaded(data)),
                            Err(error) => console::error_1(&JsValue::from_str(&error.to_string())),
                        }
                    })
                    .into(),
                )
                .unwrap(),
        )
    }

    fn render_item(&self, project: &MiteProject) -> Html {
        html! {
            <NesField>
                <NesContainer title={project.project.id.to_string().clone()}>
                    <p>{project.project.name.clone()}</p>

                    <NesLinkButton bstate=ButtonState::Success description="Open in mite" href={format!("https://{}.mite.yo.lk/reports/projects/{}", self.props.instance, project.project.id)} />
                </NesContainer>
            </NesField>
        }
    }
}

impl Component for ProjectList {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let link_clone = link.clone();
        ProjectList {
            link,
            props: props.clone(),
            _fetch_task: ProjectList::load_data(link_clone, props.instance, props.apikey),
            projects: Vec::<MiteProject>::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ProjectsLoaded(mut data) => {
                self.projects.append(&mut data);
            }
        }
        true
    }

    fn view(&self) -> Html {
        html! {
                <>
                    { for self.projects.iter().map(|p| self.render_item(p)) }
                </>
        }
    }
}
