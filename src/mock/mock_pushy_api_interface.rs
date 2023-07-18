use async_trait::async_trait;

use crate::services::pushy_service::PushyApiInterface;

pub struct MockPushyApiService {
    pub should_succeed: bool,
}

impl MockPushyApiService {
    pub fn new(should_succeed: bool) -> Self {
        Self { should_succeed }
    }
}

#[async_trait]
impl PushyApiInterface for MockPushyApiService {
    async fn send_push_notification(
        &self,
        _device_token: &str,
        _message: &str,
    ) -> Result<(), &'static str> {
        if self.should_succeed {
            Ok(())
        } else {
            Err("Push notification failed")
        }
    }
}
