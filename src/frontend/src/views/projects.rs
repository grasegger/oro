use super::secure_view::SecureView;
//use crate::components::nes_button::NesButton;
use crate::components::nes_container::NesContainer;
use web_sys::console;
use yew::{html, Component, ComponentLink, Html, ShouldRender};
use crate::mite::support_projects::MiteProject;
use yew::services::fetch::Response;
use yew::format::Json;
use yew::services::fetch::FetchService;
use yew::services::fetch::FetchTask;
use wasm_bindgen::JsValue;

pub struct Projects {
    link: ComponentLink<Self>,
    _fetch_task: Option<FetchTask>,
    projects: Vec<MiteProject>
}

pub enum Msg {
   ProjectsLoaded(Vec<MiteProject>),
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
            <p>{project.project.name.clone()}</p>
        }
    }
}

impl SecureView for Projects {
    fn render_secured_view(&self) -> Html {
        html! {
            <NesContainer title="Support-Projekte">
                <>
                {self.projects.len()}
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
            projects: Vec::<MiteProject>::new()
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ProjectsLoaded (mut data) => {
                self.projects.append(&mut data);
                console::log_1(&self.projects.len().to_string().into());
            }
        }
        true
    }

    fn view(&self) -> Html {
        self.secured_view()
    }
}
