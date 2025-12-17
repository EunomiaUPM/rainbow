use crate::entities::policy_templates::{NewPolicyTemplateDto, PolicyTemplateEntityTrait};
use crate::grpc::api::catalog_agent::policy_template_entity_service_server::PolicyTemplateEntityService;
use crate::grpc::api::catalog_agent::{
    CreatePolicyTemplateRequest, DeleteByIdRequest, GetAllRequest, GetBatchRequest, GetByIdRequest, PolicyTemplate,
    PolicyTemplateListResponse, PolicyTemplateResponse,
};
use std::str::FromStr;
use std::sync::Arc;
use tonic::{Request, Response, Status};
use urn::Urn;

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
        let req = request.into_inner();
        let templates = self
            .service
            .get_all_policy_templates(req.limit, req.page)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        let proto_templates: Vec<PolicyTemplate> = templates.into_iter().map(Into::into).collect();

        Ok(Response::new(PolicyTemplateListResponse {
            policy_templates: proto_templates,
        }))
    }

    async fn get_batch_policy_templates(
        &self,
        request: Request<GetBatchRequest>,
    ) -> Result<Response<PolicyTemplateListResponse>, Status> {
        let req = request.into_inner();
        let urns: Vec<Urn> = req
            .ids
            .iter()
            .map(|id| Urn::from_str(id))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| Status::invalid_argument("One or more IDs are invalid URNs"))?;

        let templates =
            self.service.get_batch_policy_templates(&urns).await.map_err(|e| Status::internal(e.to_string()))?;

        let proto_templates: Vec<PolicyTemplate> = templates.into_iter().map(Into::into).collect();

        Ok(Response::new(PolicyTemplateListResponse {
            policy_templates: proto_templates,
        }))
    }

    async fn get_policy_template_by_id(
        &self,
        request: Request<GetByIdRequest>,
    ) -> Result<Response<PolicyTemplateResponse>, Status> {
        let req = request.into_inner();
        let urn = Urn::from_str(&req.id).map_err(|_| Status::invalid_argument("Invalid URN"))?;

        let template_opt =
            self.service.get_policy_template_by_id(&urn).await.map_err(|e| Status::internal(e.to_string()))?;

        match template_opt {
            Some(dto) => Ok(Response::new(PolicyTemplateResponse {
                policy_template: Some(dto.into()),
            })),
            None => Err(Status::not_found("Policy Template not found")),
        }
    }

    async fn create_policy_template(
        &self,
        request: Request<CreatePolicyTemplateRequest>,
    ) -> Result<Response<PolicyTemplateResponse>, Status> {
        let req = request.into_inner();
        let new_template_dto: NewPolicyTemplateDto = req.try_into()?;

        let created_dto = self
            .service
            .create_policy_template(&new_template_dto)
            .await
            .map_err(|e| Status::internal(format!("Failed to create Policy Template: {}", e)))?;

        Ok(Response::new(PolicyTemplateResponse {
            policy_template: Some(created_dto.into()),
        }))
    }

    async fn delete_policy_template_by_id(&self, request: Request<DeleteByIdRequest>) -> Result<Response<()>, Status> {
        let req = request.into_inner();
        let urn = Urn::from_str(&req.id).map_err(|_| Status::invalid_argument("Invalid URN"))?;

        self.service
            .delete_policy_template_by_id(&urn)
            .await
            .map_err(|e| Status::internal(format!("Failed to delete Policy Template: {}", e)))?;

        Ok(Response::new(()))
    }
}
