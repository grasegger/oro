use super::secure_view::SecureView;
use crate::components::nes_link_button::NesLinkButton;
use crate::components::nes_link_button::ButtonState;
use crate::components::nes_container::NesContainer;
use crate::components::nes_field::NesField;
use web_sys::console;
use yew::{html, Component, ComponentLink, Html, ShouldRender};
use crate::mite::support_projects::MiteProject;
use crate::mite::customer::MiteCustomer;
use yew::services::fetch::Response;
use yew::format::Json;
use yew::services::fetch::FetchService;
use yew::services::fetch::FetchTask;
use wasm_bindgen::JsValue;

pub struct Projects {
    link: ComponentLink<Self>,
    _fetch_task: Option<FetchTask>,
    projects: Vec<MiteProject>,
    customer_fetch_tasks: Vec<FetchTask>,
}

pub enum Msg {
   ProjectsLoaded(Vec<MiteProject>),
   CustomerLoaded(MiteCustomer),
   ProjectsParsed,
}

impl Projects {
    fn load_data(link: ComponentLink<Self>) -> Option<FetchTask> {

        let request =  MiteProject::get_support_projects(Projects::get_instance(), Projects::get_api_key());
        Some (FetchService::new()
              .fetch(
                  request,
                  (move |response: Response<Json<anyhow::Result<Vec<MiteProject>>>>| match response
                   .into_body()
                   .0
                   {
                       Ok(data) => link.send_message(Msg::ProjectsLoaded(data)),
                       Err(error) => console::error_1( &JsValue::from_str(&error.to_string()))
                   })
                  .into(),
                  )
              .unwrap())
    }


    fn render_item(project : & MiteProject) -> Html
    {
        html! {
            <NesField>
                <NesContainer title={project.project.id.to_string().clone()}>
                    <p>{project.project.name.clone()}</p>

                    <NesLinkButton bstate=ButtonState::Success description="Open in mite" href={format!("https://{}.mite.yo.lk/project/{}", Projects::get_instance(), project.project.id)} />
                </NesContainer>
            </NesField>
        }
    }
}

impl SecureView for Projects {
    fn render_secured_view(&self) -> Html {
        html! {
            <NesContainer title="Support-Projekte">
                <>
                    { for self.projects.iter().map(|p| Projects::render_item(p)) }
                </>
            </NesContainer>
        }
    } }

impl Component for Projects {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let link_clone = link.clone();
        Projects { 
            link, 
            _fetch_task: Projects::load_data(link_clone),
            projects: Vec::<MiteProject>::new(),
            customer_fetch_tasks: Vec::<FetchTask>::new()
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ProjectsLoaded (mut data) => {
                self.projects.append(&mut data);
                self.link.send_message(Msg::ProjectsParsed);
            }
            Msg::ProjectsParsed => {
                self.projects.into_iter().map(                 |p|                     {
                    let request = p.project.get_client_for_project(Projects::get_instance(), Projects::get_api_key());
                    self.customer_fetch_tasks.push(
                        FetchService::new()
                        .fetch(
                            request,
                            (move |response: Response<Json<anyhow::Result<MiteCustomer>>>| match response
                             .into_body()
                             .0
                             {
                                 Ok(data) => self.link.clone().send_message(Msg::CustomerLoaded(data)),
                                 Err(error) => console::error_1( &JsValue::from_str(&error.to_string()))
                             })
                            .into()
                            )
                        .unwrap()
                        )
                }                )
            }
            Msg::CustomerLoaded(customer) => {
                console::log_1(&format!("{:?}", customer).into());
            }
        }
        true
    }

    fn view(&self) -> Html {
        self.secured_view()
    }
}
