use crate::models::payment_request::Payment;
use crate::services::pushy_service::PushyApiInterface;
use crate::services::reth_service::EthereumInterface;

pub struct PaymentService {
    ethereum_service: Box<dyn EthereumInterface + Send + Sync>,
    notification_api_service: Box<dyn PushyApiInterface + Send + Sync>,
}

impl PaymentService {
    pub fn new(
        ethereum_service: Box<dyn EthereumInterface + Send + Sync>,
        notification_api_service: Box<dyn PushyApiInterface + Send + Sync>,
    ) -> Self {
        Self {
            ethereum_service,
            notification_api_service,
        }
    }

    pub async fn process_payment(
        &self,
        sender: &str,
        receiver: &str,
        amount: u64,
        device_token: &str,
    ) -> Result<Payment, &'static str> {
        let transaction_id = self
            .ethereum_service
            .create_transaction(sender, receiver, amount)
            .await?;

        // Transaction creation succeeded, send push notification
        self.notification_api_service
            .send_push_notification(
                device_token,
                &format!("Transaction {} created", transaction_id),
            )
            .await?;

        // TODO: Listen for transaction confirmation and send another push notification

        Ok(Payment {
            sender: String::from(sender),
            receiver: String::from(receiver),
            amount,
            transaction_id,
        })
    }
}
