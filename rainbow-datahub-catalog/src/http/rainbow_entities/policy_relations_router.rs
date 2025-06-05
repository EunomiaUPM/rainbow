use crate::core::datahub_proxy::datahub_proxy_types::{DatasetsQueryOptions, DomainsQueryOptions};
use crate::core::datahub_proxy::DatahubProxyTrait;
use crate::core::rainbow_entities::PolicyTemplatesToDatahubDatasetRelationTrait;
use axum::extract::{Path, Query, State};
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use reqwest::StatusCode;
use std::sync::Arc;
use tracing::info;

pub struct RainbowDatahubPolicyRelationsRouter<T, U>
where
    T: DatahubProxyTrait + Send + Sync + 'static,
    U: PolicyTemplatesToDatahubDatasetRelationTrait + Send + Sync + 'static,
{
    datahub_service: Arc<T>,
    policy_relations_service: Arc<U>,
}


impl<T, U> RainbowDatahubPolicyRelationsRouter<T, U>
where
    T: DatahubProxyTrait + Send + Sync,
    U: PolicyTemplatesToDatahubDatasetRelationTrait + Send + Sync,
{
    pub fn new(datahub_service: Arc<T>, policy_relations_service: Arc<U>) -> Self {
        Self {
            datahub_service,
            policy_relations_service,
        }
    }
    pub fn router(self) -> Router {
        Router::new()
            .route("/api/v1/datahub/hola-pablo", get(Self::handle_your_routes_here))
            .with_state((self.datahub_service, self.policy_relations_service))
    }
    async fn handle_your_routes_here(
        State((datahub_service, policy_relations_service)): State<(Arc<T>, Arc<U>)>,
    ) -> impl IntoResponse {
        info!("GET /api/v1/datahub/hola-pablo");
        match datahub_service.get_datahub_domains().await {
            Ok(domains) => (StatusCode::OK, Json(domains)).into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        }
    }
}

// use crate::core::datahub_proxy::DatahubProxyTrait;
use rainbow_db::datahub::repo::{NewPolicyTemplateModel, PolicyTemplatesRepo, PolicyTemplatesRepoErrors};
// use axum::extract::State;
// use axum::response::IntoResponse;
use axum::routing::post;
// use axum::{Json, Router};
// use reqwest::StatusCode;
use serde::Deserialize;
use serde_json::json;
// use std::sync::Arc;
// use tracing::info;


pub struct PolicyTemplatesRouter<T>
where
    T: PolicyTemplatesRepo + Send + Sync + 'static,
{
    policy_templates_service: Arc<T>,
}

#[derive(Debug, Deserialize)]
pub struct CreatePolicyTemplateRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub content: serde_json::Value,
}

impl<T> PolicyTemplatesRouter<T>
where
    T: PolicyTemplatesRepo + Send + Sync,
{
    pub fn new(policy_templates_service: Arc<T>) -> Self {
        Self {
            policy_templates_service,
        }
    }

    pub fn router(self) -> Router {
        Router::new()
            .route("/api/v1/datahub/policy-templates", post(Self::create_policy_template))
            .with_state(self.policy_templates_service)
    }

    async fn create_policy_template(
        State(policy_templates_service): State<Arc<T>>,
        Json(payload): Json<CreatePolicyTemplateRequest>,
    ) -> impl IntoResponse {
        info!("POST /api/v1/datahub/policy-templates");
        
        let new_template = NewPolicyTemplateModel {
            title: payload.title,
            description: payload.description,
            content: payload.content,
        };

        match policy_templates_service.create_policy_template(new_template).await {
        Ok(template) => (StatusCode::CREATED, Json(serde_json::to_value(template).unwrap())),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e.to_string() })),
        ),
        }
    }
}