use serde::{Deserialize, Serialize};
use yew::services::fetch::Request;
use yew::format::nothing::Nothing;

#[derive(Debug, Serialize, Deserialize)]
pub struct MiteAccount {
    account: MiteAccountDetails,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MiteAccountDetails {
    id: i32,
    name: String,
    title: String
}

impl MiteAccount {
    pub fn get_account(instance: String, apikey: String) -> Request<Nothing> {
        let url = "https://corsapi.mite.yo.lk/account.json";

        Request::get(url)
            .header("X-MiteAccount", instance)
            .header("X-MiteApiKey", apikey)
            .body(Nothing)
            .expect("Failed to build request.")
    }
}
