use async_trait::async_trait;

#[async_trait]
pub trait PushyApiInterface {
    async fn send_push_notification(
        &self,
        user_id: &str,
        notification: &str,
    ) -> Result<(), &'static str>;
}
