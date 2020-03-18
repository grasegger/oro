use serde::{Deserialize, Serialize};
use yew::format::nothing::Nothing;
use yew::services::fetch::Request;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MiteProject {
    pub project: MiteProjectDetails,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MiteProjectDetails {
    pub id: i64,
    pub name: String,
    pub budget: i64,
    pub note: String,
    pub customer_id: Option<i64>,
    pub customer: Option<bool>,
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

impl MiteProjectDetails {
    pub fn get_client_for_project(&self, instance: String, apikey: String) -> Request<Nothing> {
        let url = format!(
            "https://corsapi.mite.yo.lk/customers/{}.json",
            self.customer_id.unwrap()
        );

        Request::get(url)
            .header("X-MiteAccount", instance)
            .header("X-MiteApiKey", apikey)
            .body(Nothing)
            .expect("Failed to build request.")
    }
}
