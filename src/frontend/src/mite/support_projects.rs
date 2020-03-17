use serde::{Deserialize, Serialize};
use yew::services::fetch::Request;
use yew::format::nothing::Nothing;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MiteProject {
    pub project: MiteProjectDetails,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MiteProjectDetails {
    id: i64,
    pub name: String,
    budget: i64,
    note: String,
    customer_id: Option<i64>
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
}
