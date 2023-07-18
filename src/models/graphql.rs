use async_graphql::{Context, EmptySubscription, Object, Schema};

use crate::mock::mock_ethereum_service::MockEthereumService;
use crate::mock::mock_pushy_api_interface::MockPushyApiService;
use crate::models::chat_message::{ChatMessage, ChatMessageRequest};
use crate::models::payment_request::{Payment, PaymentRequest};
use crate::services::payment_service::PaymentService;

pub struct QueryRoot;

pub struct MutationRoot;

#[Object]
impl QueryRoot {
    async fn payment(&self, _ctx: &Context<'_>, transaction_id: String) -> Payment {
        Payment {
            sender: String::from("Alice"),
            receiver: String::from("Bob"),
            amount: 100,
            transaction_id,
        }
    }

    async fn messages(&self, _ctx: &Context<'_>) -> Vec<ChatMessage> {
        vec![]
    }
}

#[Object]
impl MutationRoot {
    async fn make_payment(&self, _ctx: &Context<'_>, payment: PaymentRequest) -> Payment {
        let service = PaymentService::new(
            Box::new(MockEthereumService::new(
                Some("123".parse().unwrap()),
                Some(true),
            )),
            Box::new(MockPushyApiService::new(true)),
        );

        match service
            .process_payment(
                &payment.sender,
                &payment.receiver,
                payment.amount,
                &payment.device_token,
            )
            .await
        {
            Ok(p) => p,
            Err(e) => panic!("{}", e.to_string()),
        }
    }

    async fn send_message(&self, _ctx: &Context<'_>, message: ChatMessageRequest) -> ChatMessage {
        ChatMessage {
            id: String::from("123"),
            timestamp: String::from("123"),
            author_id: message.author_id,
            chat_id: message.chat_id,
            message_type: message.message_type,
            content: message.content,
        }
    }
}

pub(crate) type ServiceSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;
