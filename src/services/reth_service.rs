use async_trait::async_trait;

#[async_trait]
pub trait EthereumInterface {
    async fn create_transaction(
        &self,
        sender: &str,
        receiver: &str,
        amount: u64,
    ) -> Result<String, &'static str>;

    // Instead of returning a Result, we'll now return a Future
    fn listen_for_confirmation(
        &self,
        transaction_id: String,
    ) -> Box<dyn std::future::Future<Output = Result<bool, &'static str>> + Send + '_>;
}
