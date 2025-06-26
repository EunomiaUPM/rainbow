use crate::core::vc_request_service::vc_request_types::VCRequest;
use crate::core::vc_request_service::VCRequestTrait;
use crate::data::entities::vc_requests;
use crate::data::entities::vc_requests::Model;
use crate::data::repo::{EditVCRequestModel, NewVCRequestModel, VCRequestsRepo, VCRequestsRepoErrors};
use anyhow::anyhow;
use axum::async_trait;
use std::sync::Arc;
use urn::Urn;

pub struct VCRequestService<T>
where
    T: VCRequestsRepo + Send + Sync,
{
    repo: Arc<T>,
}

impl<T> VCRequestService<T>
where
    T: VCRequestsRepo + Send + Sync,
{
    pub fn new(repo: Arc<T>) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl<T> VCRequestTrait for VCRequestService<T>
where
    T: VCRequestsRepo + Send + Sync,
{
    async fn get_all_vc_requests(&self) -> anyhow::Result<Vec<vc_requests::Model>> {
        let vc_requests = self.repo.get_all_vc_requests(None, None).await.map_err(|e| anyhow!(e.to_string()))?;
        Ok(vc_requests)
    }

    async fn get_vc_request_by_id(&self, vc_request_id: Urn) -> anyhow::Result<vc_requests::Model> {
        let vc_requests = self.repo.get_all_vc_request_by_id(vc_request_id).await.map_err(|e| match e {
            VCRequestsRepoErrors::VCRequestNotFound => anyhow!("not found".to_string()),
            e => anyhow!(e.to_string()),
        })?;
        Ok(vc_requests)
    }

    async fn validate_vc_request_by_id(&self, vc_request_id: Urn) -> anyhow::Result<vc_requests::Model> {
        let vc_requests = self
            .repo
            .put_vc_request(
                vc_request_id,
                EditVCRequestModel { state: Some("Validated".to_string()) },
            )
            .await
            .map_err(|e| match e {
                VCRequestsRepoErrors::VCRequestNotFound => anyhow!("not found".to_string()),
                e => anyhow!(e.to_string()),
            })?;
        Ok(vc_requests)
    }

    async fn reject_vc_request_by_id(&self, vc_request_id: Urn) -> anyhow::Result<vc_requests::Model> {
        let vc_requests = self
            .repo
            .put_vc_request(
                vc_request_id,
                EditVCRequestModel { state: Some("Rejected".to_string()) },
            )
            .await
            .map_err(|e| match e {
                VCRequestsRepoErrors::VCRequestNotFound => anyhow!("not found".to_string()),
                e => anyhow!(e.to_string()),
            })?;
        Ok(vc_requests)
    }

    async fn create_vc_request(&self, input: VCRequest) -> anyhow::Result<Model> {
        let vc_requests = self
            .repo
            .create_vc_request(NewVCRequestModel { vc_content: input.vc_content, state: Some("Pending".to_string()) })
            .await
            .map_err(|e| anyhow!(e.to_string()))?;
        Ok(vc_requests)
    }
}
