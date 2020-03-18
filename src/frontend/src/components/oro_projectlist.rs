use crate::components::nes_container::NesContainer;
use serde::{Serialize, Deserialize};
use crate::components::nes_field::NesField;
use crate::components::nes_link_button::ButtonState;
use crate::components::nes_link_button::NesLinkButton;
use crate::mite::support_projects::MiteProject;
use crate::support::Contract;
use wasm_bindgen::JsValue;
use web_sys::console;
use yew::format::Json;
use yew::services::fetch::FetchService;
use yew::services::fetch::FetchTask;
use yew::services::fetch::Response;
use yew::{html, Properties, Component, ComponentLink, Html, ShouldRender};
use crate::components::oro_customer::Customer;
use crate::support::SupportPackage;

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
    
    fn render_addon(&self, package: SupportPackage) -> Html {
        html! {
                <NesField>
                    <NesContainer title={package.pkgname}>
                        <p>{format!("CMS: {}", package.cms)}</p>
                        <p>{format!("Booked: {}hours", package.hours)}</p>
                        <p>{format!("Starting in Q{}", package.startQuarter)}</p>
                        <p>{format!("Ending in in Q{}", package.stopQuarter)}</p>
                        <p>{format!("Yearly: {}", package.yearly)}</p>
                    </NesContainer>
                </NesField>
        }
    }

    fn render_item(&self, project: &MiteProject) -> Html {
        if project.project.note.len() < 1 {
            html!{
            }
        } else {
            let contract: Contract = serde_json::from_str(&project.project.note).unwrap();
            html! {
                <NesField>
                    <NesContainer title={project.project.name.clone()}>
                        <h2>{contract.variant.title}</h2>
                        <Customer customer_id={project.project.customer_id.unwrap().to_string()} apikey={self.props.apikey.clone()} instance={self.props.instance.clone()} />
                <NesField>
                    <NesContainer title="Base Package">
                        <p>{format!("Package Name: {}", contract.variant.pkgname)}</p>
                        <p>{format!("CMS: {}", contract.variant.cms)}</p>
                        <p>{format!("Booked: {}hours", contract.variant.hours)}</p>
                        <p>{format!("Starting in Q{}", contract.variant.startQuarter)}</p>
                        <p>{format!("Ending in in Q{}", contract.variant.stopQuarter)}</p>
                        <p>{format!("Add carry: {}hours", contract.variant.addCarry)}</p>
                    </NesContainer>
                </NesField>

                <NesField>
                    <NesContainer title={format!("{} Addon(s)", contract.addons.len())}>
                    {
                        if contract.addons.len() > 0 {
                            html! {
                                { for contract.addons.iter().map(|c| self.render_addon(c.clone())) }
                            }
                        } else {
                            html! {
                                <p>{"No Addons booked."}</p> 
                            }
                        }
                    }
                </NesContainer>
                    </NesField>
                    <NesLinkButton bstate=ButtonState::Success description="Open in mite" href={format!("https://{}.mite.yo.lk/reports/projects/{}", self.props.instance, project.project.id)} />
                    </NesContainer>
                    </NesField>
            }
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
