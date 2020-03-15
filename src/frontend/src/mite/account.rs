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
    pub fn getAccount(instance: String, apikey: String) -> Request<Nothing> {
           let url = format!("https://{}.mite.yo.lk/account.json", instance);

          Request::get(url)
               .header("X-MiteApiKey", apikey)
               .header("User-Agent", "oro/0.0.0 (https://github.com/grasegger/oro) custom")
               .body(Nothing)
               .expect("Failed to build request.")
    }
}
