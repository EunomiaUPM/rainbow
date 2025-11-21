use crate::entities::transfer_messages::TransferAgentMessagesTrait;
use crate::grpc::api::transfer_messages::transfer_agent_messages_server::TransferAgentMessages;
use crate::grpc::api::transfer_messages::{
    CreateMessageRequest, PaginationRequestMessages, ResourceIdRequestMessages, TransferMessageListResponse,
    TransferMessageResponse,
};
use crate::http::transfer_messages::PaginationParams;
use rainbow_common::config::provider_config::ApplicationProviderConfig;
use std::sync::Arc;
use tonic::{Request, Response, Status};
mod mappers;

pub struct TransferAgentMessagesGrpc {
    service: Arc<dyn TransferAgentMessagesTrait>,
    config: Arc<ApplicationProviderConfig>,
}

impl TransferAgentMessagesGrpc {
    pub fn new(service: Arc<dyn TransferAgentMessagesTrait>, config: Arc<ApplicationProviderConfig>) -> Self {
        Self { service, config }
    }
}

#[tonic::async_trait]
impl TransferAgentMessages for TransferAgentMessagesGrpc {
    async fn get_all_messages(
        &self,
        request: Request<PaginationRequestMessages>,
    ) -> Result<Response<TransferMessageListResponse>, Status> {
        let proto_req = request.into_inner();
        let params: PaginationParams = proto_req.into();
        let messages = self
            .service
            .get_all_transfer_messages(params.limit, params.page)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;
        let proto_messages = messages
            .into_iter()
            .map(|m| m.into()) // Llama a From<TransferMessageDto>
            .collect();
        Ok(Response::new(TransferMessageListResponse {
            messages: proto_messages,
        }))
    }

    async fn create_message(
        &self,
        request: Request<CreateMessageRequest>,
    ) -> Result<Response<TransferMessageResponse>, Status> {
        todo!()
    }

    async fn get_message_by_id(
        &self,
        request: Request<ResourceIdRequestMessages>,
    ) -> Result<Response<TransferMessageResponse>, Status> {
        todo!()
    }

    async fn delete_message(&self, request: Request<ResourceIdRequestMessages>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn get_messages_by_process_id(
        &self,
        request: Request<ResourceIdRequestMessages>,
    ) -> Result<Response<TransferMessageListResponse>, Status> {
        todo!()
    }
}
