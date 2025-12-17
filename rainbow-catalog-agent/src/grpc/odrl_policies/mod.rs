use crate::entities::odrl_policies::OdrlPolicyEntityTrait;
use crate::grpc::api::catalog_agent::odrl_policy_entity_service_server::OdrlPolicyEntityService;
use crate::grpc::api::catalog_agent::{
    CreateOdrlPolicyRequest, DeleteByEntityIdRequest, DeleteByIdRequest, GetAllRequest, GetBatchRequest,
    GetByEntityIdRequest, GetByIdRequest, OdrlPolicyListResponse, OdrlPolicyResponse,
};
use std::sync::Arc;
use tonic::{Request, Response, Status};

pub struct OdrlPolicyEntityGrpc {
    service: Arc<dyn OdrlPolicyEntityTrait>,
}

impl OdrlPolicyEntityGrpc {
    pub fn new(service: Arc<dyn OdrlPolicyEntityTrait>) -> Self {
        Self { service }
    }
}

#[tonic::async_trait]
impl OdrlPolicyEntityService for OdrlPolicyEntityGrpc {
    async fn get_all_odrl_offers(
        &self,
        request: Request<GetAllRequest>,
    ) -> Result<Response<OdrlPolicyListResponse>, Status> {
        todo!()
    }

    async fn get_batch_odrl_offers(
        &self,
        request: Request<GetBatchRequest>,
    ) -> Result<Response<OdrlPolicyListResponse>, Status> {
        todo!()
    }

    async fn get_all_odrl_offers_by_entity(
        &self,
        request: Request<GetByEntityIdRequest>,
    ) -> Result<Response<OdrlPolicyListResponse>, Status> {
        todo!()
    }

    async fn get_odrl_offer_by_id(
        &self,
        request: Request<GetByIdRequest>,
    ) -> Result<Response<OdrlPolicyResponse>, Status> {
        todo!()
    }

    async fn create_odrl_offer(
        &self,
        request: Request<CreateOdrlPolicyRequest>,
    ) -> Result<Response<OdrlPolicyResponse>, Status> {
        todo!()
    }

    async fn delete_odrl_offer_by_id(&self, request: Request<DeleteByIdRequest>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn delete_odrl_offers_by_entity(
        &self,
        request: Request<DeleteByEntityIdRequest>,
    ) -> Result<Response<()>, Status> {
        todo!()
    }
}
