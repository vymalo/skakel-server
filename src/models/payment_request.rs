use async_graphql::{InputObject, SimpleObject};
use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize, SimpleObject, InputObject)]
pub(crate) struct PaymentRequest {
    pub(crate) sender: String,
    pub(crate) receiver: String,
    pub(crate) amount: u64,
    pub(crate) device_token: String,
}

#[derive(Serialize, SimpleObject)]
pub struct Payment {
    pub(crate) transaction_id: String,
    pub(crate) sender: String,
    pub(crate) receiver: String,
    pub(crate) amount: u64,
}
