use async_graphql::{InputObject, SimpleObject};
use serde::Deserialize;

#[derive(Deserialize, SimpleObject, InputObject)]
pub struct ChatMessageRequest {
    pub author_id: String,
    pub chat_id: String,
    pub message_type: String,
    pub content: String,
}

#[derive(Deserialize, SimpleObject)]
pub struct ChatMessage {
    pub id: String,
    pub timestamp: String,

    pub author_id: String,
    pub chat_id: String,
    pub message_type: String,
    pub content: String,
}
