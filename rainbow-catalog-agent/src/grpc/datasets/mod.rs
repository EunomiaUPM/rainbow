use crate::entities::datasets::DatasetEntityTrait;
use crate::grpc::api::catalog_agent::dataset_entity_service_server::DatasetEntityService;
use crate::grpc::api::catalog_agent::{
    CreateDatasetRequest, DatasetListResponse, DatasetResponse, DeleteByIdRequest, GetAllRequest, GetBatchRequest,
    GetByIdRequest, GetByParentIdRequest, PutDatasetRequest,
};
use std::sync::Arc;
use tonic::{Request, Response, Status};

pub struct DatasetEntityGrpc {
    service: Arc<dyn DatasetEntityTrait>,
}

impl DatasetEntityGrpc {
    pub fn new(service: Arc<dyn DatasetEntityTrait>) -> Self {
        Self { service }
    }
}

#[tonic::async_trait]
impl DatasetEntityService for DatasetEntityGrpc {
    async fn get_all_datasets(&self, request: Request<GetAllRequest>) -> Result<Response<DatasetListResponse>, Status> {
        todo!()
    }

    async fn get_batch_datasets(
        &self,
        request: Request<GetBatchRequest>,
    ) -> Result<Response<DatasetListResponse>, Status> {
        todo!()
    }

    async fn get_datasets_by_catalog_id(
        &self,
        request: Request<GetByParentIdRequest>,
    ) -> Result<Response<DatasetListResponse>, Status> {
        todo!()
    }

    async fn get_dataset_by_id(&self, request: Request<GetByIdRequest>) -> Result<Response<DatasetResponse>, Status> {
        todo!()
    }

    async fn create_dataset(
        &self,
        request: Request<CreateDatasetRequest>,
    ) -> Result<Response<DatasetResponse>, Status> {
        todo!()
    }

    async fn put_dataset_by_id(
        &self,
        request: Request<PutDatasetRequest>,
    ) -> Result<Response<DatasetResponse>, Status> {
        todo!()
    }

    async fn delete_dataset_by_id(&self, request: Request<DeleteByIdRequest>) -> Result<Response<()>, Status> {
        todo!()
    }
}
