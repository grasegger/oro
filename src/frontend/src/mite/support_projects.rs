use serde::{Deserialize, Serialize};
use yew::services::fetch::Request;
use yew::format::nothing::Nothing;
use yew::format::Json;
use yew::services::fetch::Response;
use yew::services::fetch::FetchService;
use yew::html::ComponentLink;
use wasm_bindgen::JsValue;
//use yew::services::fetch::FetchTask;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MiteProject {
    pub project: MiteProjectDetails,
}

#[derive(Clone, Debug, Serialize, Deserialize, Json)]
pub struct MiteProjectDetails {
    pub id: i64,
    pub name: String,
    pub budget: i64,
    pub note: String,
    pub customer_id: Option<i64>,
    pub customer: Option<bool>,
    customer_fetch_task = Option<FetchTask>,
}

impl MiteProject {
    pub fn get_support_projects(instance: String, apikey: String) -> Request<Nothing> {
        let url = "https://corsapi.mite.yo.lk/projects.json?name=Supportvertrag";

        Request::get(url)
            .header("X-MiteAccount", instance)
            .header("X-MiteApiKey", apikey)
            .body(Nothing)
            .expect("Failed to build request.")
    }

    pub fn get_client_for_project (&self, instance: String, apikey: String) -> Request<Nothing> {
        let url = format!("https://corsapi.mite.yo.lk/customers/{}.json");

        Request::get(url)
            .header("X-MiteAccount", instance)
            .header("X-MiteApiKey", apikey)
            .body(Nothing)
            .expect("Failed to build request.")
    }

}
