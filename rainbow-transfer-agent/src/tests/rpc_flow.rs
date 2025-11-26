#![allow(unused)]

use crate::data::entities::transfer_message;
use crate::data::factory_trait::MockTransferAgentRepoTrait;
use crate::data::repo_traits::transfer_message_repo::MockTransferMessageRepoTrait;
use crate::entities::transfer_messages::transfer_messages::TransferAgentMessagesService;
use crate::entities::transfer_messages::MockTransferAgentMessagesTrait;
use crate::entities::transfer_process::{MockTransferAgentProcessesTrait, TransferAgentProcessesTrait};
use crate::grpc::api::transfer_messages::transfer_agent_messages_server::TransferAgentMessages;
use crate::http::transfer_messages::TransferAgentMessagesRouter;
use rainbow_common::config::provider_config::ApplicationProviderConfig;
use std::sync::Arc;

// Modificación para devolver el mock junto con el router

fn create_stub_messages() -> Vec<transfer_message::Model> {
    vec![
        transfer_message::Model {
            id: "urn:transfer-message:1".to_string(),
            transfer_agent_process_id: "urn:transfer-process:1".to_string(),
            created_at: Default::default(),
            direction: "INBOUND".to_string(),
            protocol: "DSP".to_string(),
            message_type: "TransferRequestMessage".to_string(),
            state_transition_from: "-".to_string(),
            state_transition_to: "REQUESTED".to_string(),
            payload: None,
        },
        transfer_message::Model {
            id: "urn:transfer-message:2".to_string(),
            transfer_agent_process_id: "urn:transfer-process:1".to_string(),
            created_at: Default::default(),
            direction: "OUTBOUND".to_string(),
            protocol: "DSP".to_string(),
            message_type: "TransferStart".to_string(),
            state_transition_from: "REQUESTED".to_string(),
            state_transition_to: "STARTED".to_string(),
            payload: None,
        },
    ]
}

async fn create_mock_router() -> (axum::Router, Arc<MockTransferAgentRepoTrait>) {
    let config = Arc::new(ApplicationProviderConfig::default());
    let transfer_repo = Arc::new(MockTransferAgentRepoTrait::new());
    let messages_controller_service = Arc::new(TransferAgentMessagesService::new(transfer_repo.clone()));
    let messages_router = TransferAgentMessagesRouter::new(messages_controller_service.clone(), config.clone());

    let router = axum::Router::new().merge(messages_router.router());
    (router, transfer_repo)
}

#[tokio::test]
async fn flow_transfer() -> anyhow::Result<()> {
    let (router, mock_repo_arc) = create_mock_router().await;
    let mut mock_repo = Arc::clone(&mock_repo_arc);
    let mut message_repo = Arc::new(MockTransferMessageRepoTrait::new());
    Ok(())
}
