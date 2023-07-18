use std::future::Future;

use async_trait::async_trait;

use crate::services::reth_service::EthereumInterface;

pub struct MockEthereumService {
    pub transaction_id: Option<String>,
    pub transaction_confirmation: Option<bool>,
}

impl MockEthereumService {
    pub fn new(transaction_id: Option<String>, transaction_confirmation: Option<bool>) -> Self {
        Self {
            transaction_id,
            transaction_confirmation,
        }
    }
}

#[async_trait]
impl EthereumInterface for MockEthereumService {
    async fn create_transaction(
        &self,
        _sender: &str,
        _receiver: &str,
        _amount: u64,
    ) -> Result<String, &'static str> {
        match &self.transaction_id {
            Some(id) => Ok(id.clone()),
            None => Err("Error creating transaction"),
        }
    }

    fn listen_for_confirmation(
        &self,
        _transaction_id: String,
    ) -> Box<dyn Future<Output = Result<bool, &'static str>> + Send + '_> {
        let result = self.transaction_confirmation.unwrap_or(false);
        Box::new(async move { Ok(result) })
    }
}
