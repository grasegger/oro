use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MiteCustomer {
    pub project: MiteCustomerDetails,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MiteCustomerDetails {
    pub id: i64,
    pub name: String,
}
