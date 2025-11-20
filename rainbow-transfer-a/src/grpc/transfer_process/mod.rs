mod mappers;

use crate::entities::transfer_process::TransferAgentProcessesTrait;
use std::str::FromStr;
use std::sync::Arc;
use tonic::{Request, Response, Status};
use crate::grpc::api::transfer_processes::transfer_agent_processes_server::TransferAgentProcesses;
use crate::grpc::api::transfer_processes::{GetByKeyRequest,BatchProcessRequest, CreateProcessRequest, TransferProcessResponse, UpdateProcessRequest,PaginationRequestProcesses,ResourceIdRequestProcesses,TransferProcessListResponse};

pub struct TransferAgentProcessesGrpc {
    service: Arc<dyn TransferAgentProcessesTrait>,
}

impl TransferAgentProcessesGrpc {
    pub fn new(service: Arc<dyn TransferAgentProcessesTrait>) -> Self {
        Self { service }
    }

    fn map_error(e: anyhow::Error) -> Status {
        Status::internal(e.to_string())
    }
}

#[tonic::async_trait]
impl TransferAgentProcesses for TransferAgentProcessesGrpc {
    async fn get_all_processes(
        &self,
        request: Request<PaginationRequestProcesses>,
    ) -> Result<Response<TransferProcessListResponse>, Status> {
        let proto_req = request.into_inner();
        let params: PaginationRequestProcesses = proto_req.into();
        let processes = self
            .service
            .get_all_transfer_processes(params.limit, params.page)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;
        let proto_processes = processes
            .into_iter()
            .map(|m| m.into()) // Llama a From<TransferMessageDto>
            .collect();
        Ok(Response::new(TransferProcessListResponse {
            processes: proto_processes,
        }))
    }

    async fn create_process(
        &self,
        request: Request<CreateProcessRequest>,
    ) -> Result<Response<TransferProcessResponse>, Status> {
        todo!()
    }

    async fn get_batch_processes(
        &self,
        request: Request<BatchProcessRequest>,
    ) -> Result<Response<TransferProcessListResponse>, Status> {
        todo!()
    }

    async fn get_process_by_id(
        &self,
        request: Request<ResourceIdRequestProcesses>,
    ) -> Result<Response<TransferProcessResponse>, Status> {
        todo!()
    }

    async fn update_process(
        &self,
        request: Request<UpdateProcessRequest>,
    ) -> Result<Response<TransferProcessResponse>, Status> {
        todo!()
    }

    async fn delete_process(&self, request: Request<ResourceIdRequestProcesses>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn get_process_by_key_id(
        &self,
        request: Request<GetByKeyRequest>,
    ) -> Result<Response<TransferProcessResponse>, Status> {
        todo!()
    }
}
