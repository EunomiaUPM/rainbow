use crate::entities::policy_templates::PolicyTemplateEntityTrait;
use crate::grpc::api::catalog_agent::policy_template_entity_service_server::PolicyTemplateEntityService;
use crate::grpc::api::catalog_agent::{
    CreatePolicyTemplateRequest, DeleteByIdRequest, GetAllRequest, GetBatchRequest, GetByIdRequest,
    PolicyTemplateListResponse, PolicyTemplateResponse,
};
use std::sync::Arc;
use tonic::{Request, Response, Status};

pub struct PolicyTemplateEntityGrpc {
    service: Arc<dyn PolicyTemplateEntityTrait>,
}

impl PolicyTemplateEntityGrpc {
    pub fn new(service: Arc<dyn PolicyTemplateEntityTrait>) -> Self {
        Self { service }
    }
}

#[tonic::async_trait]
impl PolicyTemplateEntityService for PolicyTemplateEntityGrpc {
    async fn get_all_policy_templates(
        &self,
        request: Request<GetAllRequest>,
    ) -> Result<Response<PolicyTemplateListResponse>, Status> {
        todo!()
    }

    async fn get_batch_policy_templates(
        &self,
        request: Request<GetBatchRequest>,
    ) -> Result<Response<PolicyTemplateListResponse>, Status> {
        todo!()
    }

    async fn get_policy_template_by_id(
        &self,
        request: Request<GetByIdRequest>,
    ) -> Result<Response<PolicyTemplateResponse>, Status> {
        todo!()
    }

    async fn create_policy_template(
        &self,
        request: Request<CreatePolicyTemplateRequest>,
    ) -> Result<Response<PolicyTemplateResponse>, Status> {
        todo!()
    }

    async fn delete_policy_template_by_id(&self, request: Request<DeleteByIdRequest>) -> Result<Response<()>, Status> {
        todo!()
    }
}
