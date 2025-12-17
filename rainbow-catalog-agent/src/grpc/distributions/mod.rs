use crate::entities::distributions::DistributionEntityTrait;
use crate::grpc::api::catalog_agent::distribution_entity_service_server::DistributionEntityService;
use crate::grpc::api::catalog_agent::{
    CreateDistributionRequest, DeleteByIdRequest, DistributionListResponse, DistributionResponse, GetAllRequest,
    GetBatchRequest, GetByIdRequest, GetByParentIdRequest, GetDistributionByFormatRequest, PutDistributionRequest,
};
use std::sync::Arc;
use tonic::{Request, Response, Status};

pub struct DistributionEntityGrpc {
    service: Arc<dyn DistributionEntityTrait>,
}

impl DistributionEntityGrpc {
    pub fn new(service: Arc<dyn DistributionEntityTrait>) -> Self {
        Self { service }
    }
}

#[tonic::async_trait]
impl DistributionEntityService for DistributionEntityGrpc {
    async fn get_all_distributions(
        &self,
        request: Request<GetAllRequest>,
    ) -> Result<Response<DistributionListResponse>, Status> {
        todo!()
    }

    async fn get_batch_distributions(
        &self,
        request: Request<GetBatchRequest>,
    ) -> Result<Response<DistributionListResponse>, Status> {
        todo!()
    }

    async fn get_distributions_by_dataset_id(
        &self,
        request: Request<GetByParentIdRequest>,
    ) -> Result<Response<DistributionListResponse>, Status> {
        todo!()
    }

    async fn get_distribution_by_dataset_and_format(
        &self,
        request: Request<GetDistributionByFormatRequest>,
    ) -> Result<Response<DistributionResponse>, Status> {
        todo!()
    }

    async fn get_distribution_by_id(
        &self,
        request: Request<GetByIdRequest>,
    ) -> Result<Response<DistributionResponse>, Status> {
        todo!()
    }

    async fn create_distribution(
        &self,
        request: Request<CreateDistributionRequest>,
    ) -> Result<Response<DistributionResponse>, Status> {
        todo!()
    }

    async fn put_distribution_by_id(
        &self,
        request: Request<PutDistributionRequest>,
    ) -> Result<Response<DistributionResponse>, Status> {
        todo!()
    }

    async fn delete_distribution_by_id(&self, request: Request<DeleteByIdRequest>) -> Result<Response<()>, Status> {
        todo!()
    }
}
