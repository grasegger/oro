use serde::{Deserialize, Serialize};
use yew::services::fetch::Request;
use yew::format::nothing::Nothing;

#[derive(Debug, Serialize, Deserialize)]
pub struct MiteAccount {
    id: i32,
    name: String,
    title: String
}

impl MiteAccount {
    pub fn get_account(instance: String, apikey: String) -> Request<Nothing> {
           let url = format!("https://{}.mite.yo.lk/account.json?api_key={}", instance, apikey);

          Request::get(url)
               .body(Nothing)
               .expect("Failed to build request.")
    }
}
