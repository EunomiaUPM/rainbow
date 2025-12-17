use crate::entities::catalogs::CatalogEntityTrait;
use crate::grpc::api::catalog_agent::catalog_entity_service_server::CatalogEntityService;
use crate::grpc::api::catalog_agent::{
    CatalogListResponse, CatalogResponse, CreateCatalogRequest, DeleteByIdRequest, GetAllCatalogsRequest,
    GetBatchRequest, GetByIdRequest, PutCatalogRequest,
};
use std::sync::Arc;
use tonic::{Request, Response, Status};

pub struct CatalogEntityGrpc {
    service: Arc<dyn CatalogEntityTrait>,
}

impl CatalogEntityGrpc {
    pub fn new(service: Arc<dyn CatalogEntityTrait>) -> Self {
        Self { service }
    }
}

#[tonic::async_trait]
impl CatalogEntityService for CatalogEntityGrpc {
    async fn get_all_catalogs(
        &self,
        request: Request<GetAllCatalogsRequest>,
    ) -> Result<Response<CatalogListResponse>, Status> {
        todo!()
    }

    async fn get_batch_catalogs(
        &self,
        request: Request<GetBatchRequest>,
    ) -> Result<Response<CatalogListResponse>, Status> {
        todo!()
    }

    async fn get_catalog_by_id(&self, request: Request<GetByIdRequest>) -> Result<Response<CatalogResponse>, Status> {
        todo!()
    }

    async fn get_main_catalog(&self, request: Request<()>) -> Result<Response<CatalogResponse>, Status> {
        todo!()
    }

    async fn create_catalog(
        &self,
        request: Request<CreateCatalogRequest>,
    ) -> Result<Response<CatalogResponse>, Status> {
        todo!()
    }

    async fn create_main_catalog(
        &self,
        request: Request<CreateCatalogRequest>,
    ) -> Result<Response<CatalogResponse>, Status> {
        todo!()
    }

    async fn put_catalog_by_id(
        &self,
        request: Request<PutCatalogRequest>,
    ) -> Result<Response<CatalogResponse>, Status> {
        todo!()
    }

    async fn delete_catalog_by_id(&self, request: Request<DeleteByIdRequest>) -> Result<Response<()>, Status> {
        todo!()
    }
}
