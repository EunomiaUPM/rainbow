use crate::entities::data_services::DataServiceEntityTrait;
use crate::grpc::api::catalog_agent::data_service_entity_service_server::DataServiceEntityService;
use crate::grpc::api::catalog_agent::{
    CreateDataServiceRequest, DataServiceListResponse, DataServiceResponse, DeleteByIdRequest, GetAllRequest,
    GetBatchRequest, GetByIdRequest, GetByParentIdRequest, PutDataServiceRequest,
};
use std::sync::Arc;
use tonic::{Request, Response, Status};

pub struct DataServiceEntityGrpc {
    service: Arc<dyn DataServiceEntityTrait>,
}

impl DataServiceEntityGrpc {
    pub fn new(service: Arc<dyn DataServiceEntityTrait>) -> Self {
        Self { service }
    }
}

#[tonic::async_trait]
impl DataServiceEntityService for DataServiceEntityGrpc {
    async fn get_all_data_services(
        &self,
        request: Request<GetAllRequest>,
    ) -> Result<Response<DataServiceListResponse>, Status> {
        todo!()
    }

    async fn get_batch_data_services(
        &self,
        request: Request<GetBatchRequest>,
    ) -> Result<Response<DataServiceListResponse>, Status> {
        todo!()
    }

    async fn get_data_services_by_catalog_id(
        &self,
        request: Request<GetByParentIdRequest>,
    ) -> Result<Response<DataServiceListResponse>, Status> {
        todo!()
    }

    async fn get_data_service_by_id(
        &self,
        request: Request<GetByIdRequest>,
    ) -> Result<Response<DataServiceResponse>, Status> {
        todo!()
    }

    async fn create_data_service(
        &self,
        request: Request<CreateDataServiceRequest>,
    ) -> Result<Response<DataServiceResponse>, Status> {
        todo!()
    }

    async fn put_data_service_by_id(
        &self,
        request: Request<PutDataServiceRequest>,
    ) -> Result<Response<DataServiceResponse>, Status> {
        todo!()
    }

    async fn delete_data_service_by_id(&self, request: Request<DeleteByIdRequest>) -> Result<Response<()>, Status> {
        todo!()
    }
}
