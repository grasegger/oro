use serde::{Deserialize, Serialize};
use yew::format::nothing::Nothing;
use yew::services::fetch::Request;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MiteCustomer {
    #[serde(rename = "customer")]
    pub details: MiteCustomerDetails,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MiteCustomerDetails {
    pub id: i64,
    pub name: String,
}

impl MiteCustomer {
    pub fn get_customer(customer_id: String, instance: String, apikey: String) -> Request<Nothing> {
        let url = format!(
            "https://corsapi.mite.yo.lk/customers/{}.json",
            customer_id
        );

        Request::get(url)
            .header("X-MiteAccount", instance)
            .header("X-MiteApiKey", apikey)
            .body(Nothing)
            .expect("Failed to build request.")
    }
}
