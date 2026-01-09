use crate::entities::policy_templates::{NewPolicyTemplateDto, PolicyTemplateEntityTrait};
use crate::errors::error_adapter::CustomToResponse;
use crate::http::common::to_camel_case::ToCamelCase;
use crate::http::common::{extract_payload, parse_urn};
use axum::extract::rejection::JsonRejection;
use axum::extract::{FromRef, Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{delete, get, post};
use axum::{Json, Router};
use rainbow_common::batch_requests::BatchRequestsAsString;
use rainbow_common::config::services::CatalogConfig;
use rainbow_common::errors::CommonErrors;
use serde::Deserialize;
use std::sync::Arc;

#[derive(Clone)]
pub struct PolicyTemplateEntityRouter {
    service: Arc<dyn PolicyTemplateEntityTrait>,
    config: Arc<CatalogConfig>,
}

#[derive(Deserialize)]
pub struct PaginationParams {
    pub limit: Option<u64>,
    pub page: Option<u64>,
}

impl FromRef<PolicyTemplateEntityRouter> for Arc<dyn PolicyTemplateEntityTrait> {
    fn from_ref(state: &PolicyTemplateEntityRouter) -> Self {
        state.service.clone()
    }
}

impl FromRef<PolicyTemplateEntityRouter> for Arc<CatalogConfig> {
    fn from_ref(state: &PolicyTemplateEntityRouter) -> Self {
        state.config.clone()
    }
}

impl PolicyTemplateEntityRouter {
    pub fn new(service: Arc<dyn PolicyTemplateEntityTrait>, config: Arc<CatalogConfig>) -> Self {
        Self { service, config }
    }

    pub fn router(self) -> Router {
        Router::new()
            .route("/", get(Self::handle_get_all_policy_templates))
            .route("/", post(Self::handle_create_policy_template))
            .route("/batch", post(Self::handle_get_batch_policy_templates))
            .route("/:id", get(Self::handle_get_policy_template_by_id))
            .route(
                "/:id/:version",
                get(Self::handle_get_policy_template_by_id_and_version),
            )
            .route(
                "/:id/:version",
                delete(Self::handle_delete_policy_template_by_id_and_version),
            )
            .with_state(self)
    }

    async fn handle_get_all_policy_templates(
        State(state): State<PolicyTemplateEntityRouter>,
        Query(params): Query<PaginationParams>,
    ) -> impl IntoResponse {
        match state.service.get_all_policy_templates(params.limit, params.page).await {
            Ok(templates) => (StatusCode::OK, Json(ToCamelCase(templates))).into_response(),
            Err(err) => err.to_response(),
        }
    }
    async fn handle_get_batch_policy_templates(
        State(state): State<PolicyTemplateEntityRouter>,
        input: Result<Json<BatchRequestsAsString>, JsonRejection>,
    ) -> impl IntoResponse {
        let input = match extract_payload(input) {
            Ok(v) => v,
            Err(e) => return e,
        };
        match state.service.get_batch_policy_templates(&input.ids).await {
            Ok(templates) => (StatusCode::OK, Json(ToCamelCase(templates))).into_response(),
            Err(err) => err.to_response(),
        }
    }
    async fn handle_get_policy_template_by_id(
        State(state): State<PolicyTemplateEntityRouter>,
        Path(id): Path<String>,
    ) -> impl IntoResponse {
        match state.service.get_policies_template_by_id(&id).await {
            Ok(templates) => (StatusCode::OK, Json(ToCamelCase(templates))).into_response(),
            Err(err) => err.to_response(),
        }
    }
    async fn handle_get_policy_template_by_id_and_version(
        State(state): State<PolicyTemplateEntityRouter>,
        Path((id, version)): Path<(String, String)>,
    ) -> impl IntoResponse {
        match state.service.get_policies_template_by_version_and_id(&id, &version).await {
            Ok(Some(template)) => (StatusCode::OK, Json(ToCamelCase(template))).into_response(),
            Ok(None) => {
                let err = CommonErrors::missing_resource_new(id.as_str(), "Policy template not found");
                err.into_response()
            }
            Err(err) => err.to_response(),
        }
    }
    async fn handle_create_policy_template(
        State(state): State<PolicyTemplateEntityRouter>,
        input: Result<Json<NewPolicyTemplateDto>, JsonRejection>,
    ) -> impl IntoResponse {
        let input = match extract_payload(input) {
            Ok(v) => v,
            Err(e) => return e,
        };
        match state.service.create_policy_template(&input).await {
            Ok(template) => (StatusCode::OK, Json(ToCamelCase(template))).into_response(),
            Err(err) => err.to_response(),
        }
    }
    async fn handle_delete_policy_template_by_id_and_version(
        State(state): State<PolicyTemplateEntityRouter>,
        Path((id, version)): Path<(String, String)>,
    ) -> impl IntoResponse {
        match state.service.delete_policy_template_by_version_and_id(&id, &version).await {
            Ok(_) => StatusCode::ACCEPTED.into_response(),
            Err(err) => match err.downcast::<CommonErrors>() {
                Ok(ce) => match ce {
                    CommonErrors::DatabaseError { ref cause, .. } => {
                        if cause.contains("not found") {
                            let err = CommonErrors::missing_resource_new("", cause.as_str());
                            return err.into_response();
                        } else {
                            ce.into_response()
                        }
                    }
                    e => return e.into_response(),
                },
                Err(e) => e.to_response(),
            },
        }
    }
}
