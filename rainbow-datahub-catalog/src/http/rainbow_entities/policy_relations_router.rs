use crate::core::datahub_proxy::datahub_proxy_types::{DatasetsQueryOptions, DomainsQueryOptions};
use crate::core::datahub_proxy::DatahubProxyTrait;
use crate::core::rainbow_entities::PolicyTemplatesToDatahubDatasetRelationTrait;
use axum::extract::{Path, Query, State};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use reqwest::StatusCode;
use std::{sync::Arc, str::FromStr}; 
use tracing::info;
use tracing::error;
use serde::Deserialize;
use serde_json::json;
use urn::Urn;
use rainbow_db::datahub::repo::NewPolicyRelationModel;

#[derive(Debug, Deserialize)]
pub struct CreatePolicyRelationRequest {
    pub dataset_id: String,
    pub policy_template_id: String,
    pub extra_content: Option<serde_json::Value>,
}

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
            .route("/api/v1/datahub/policy-relations", post(Self::create_policy_relation))
            .with_state((self.datahub_service, self.policy_relations_service))
    }

    async fn create_policy_relation(
        State((datahub_service, policy_relations_service)): State<(Arc<T>, Arc<U>)>,
        Json(payload): Json<CreatePolicyRelationRequest>,
    ) -> impl IntoResponse {
        info!("POST /api/v1/datahub/policy-relations");
        
        let new_relation = NewPolicyRelationModel {
            dataset_id: payload.dataset_id,
            policy_template_id: payload.policy_template_id,
            extra_content: payload.extra_content,
        };
        
        match policy_relations_service.create_policy_relation(new_relation).await {
            Ok(relation) => (StatusCode::CREATED, Json(serde_json::to_value(relation).unwrap())),
            Err(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": e.to_string() })),
            ),
        }
    }

}


// use crate::core::datahub_proxy::DatahubProxyTrait;
use rainbow_db::datahub::repo::{NewPolicyTemplateModel, PolicyTemplatesRepo, PolicyTemplatesRepoErrors};
// use axum::extract::State;
// use axum::response::IntoResponse;
use axum::routing::{delete};
// use axum::{Json, Router};
// use reqwest::StatusCode;

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

#[derive(Debug, Deserialize)]
pub struct GetPolicyTemplatesParams {
    #[serde(default = "default_limit")]
    pub limit: Option<u64>,
    #[serde(default = "default_page")]
    pub page: Option<u64>,
}

fn default_limit() -> Option<u64> {
    Some(10)
}

fn default_page() -> Option<u64> {
    Some(1)
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
            .route("/api/v1/datahub/policy-templates/:id", delete(Self::delete_policy_template_by_id))
            .route("/api/v1/datahub/policy-templates", get(Self::get_all_policy_templates))
            .route("/api/v1/datahub/policy-templates/:id", get(Self::get_policy_template_by_id))
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

    async fn delete_policy_template_by_id(
        State(policy_templates_service): State<Arc<T>>,
        Path(id): Path<String>,
    ) -> impl IntoResponse {
        info!("DELETE /api/v1/datahub/policy-templates/{}", id);
        
        match policy_templates_service.delete_policy_template_by_id(id).await {
            Ok(_) => {
                info!("Policy template eliminada exitosamente");
                (
                    StatusCode::NO_CONTENT,
                    Json(json!({ "message": "Policy template deleted successfully" }))
                )
            },
            Err(e) => {
                error!("Error al eliminar policy template: {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "error": e.to_string() }))
                )
            },
        }
    }

    async fn get_all_policy_templates(
        State(policy_templates_service): State<Arc<T>>,
        Query(params): Query<GetPolicyTemplatesParams>,  // Añadimos los parámetros de query
    ) -> impl IntoResponse {
        info!("GET /api/v1/datahub/policy-templates");
        
        match policy_templates_service.get_all_policy_templates(params.limit, params.page).await {
            Ok(templates) => {
                info!("Policy templates obtenidas exitosamente");
                (
                    StatusCode::OK,
                    Json(json!({ "templates": templates }))
                )
            },
            Err(e) => {
                error!("Error al obtener policy templates: {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "error": e.to_string() }))
                )
            },
        }
    }

    async fn get_policy_template_by_id(
        State(policy_templates_service): State<Arc<T>>,
        Path(id): Path<String>,
    ) -> impl IntoResponse {
        info!("GET /api/v1/datahub/policy-templates/{}", id);
        
        match policy_templates_service.get_policy_template_by_id(id).await {
            Ok(Some(template)) => {
                info!("Policy template encontrada exitosamente");
                (
                    StatusCode::OK,
                    Json(json!({ "template": template }))
                )
            },
            Ok(None) => {
                info!("Policy template no encontrada");
                (
                    StatusCode::NOT_FOUND,
                    Json(json!({ "error": "Policy template not found" }))
                )
            },
            Err(e) => {
                error!("Error al obtener policy template: {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "error": e.to_string() }))
                )
            },
        }
    }
}