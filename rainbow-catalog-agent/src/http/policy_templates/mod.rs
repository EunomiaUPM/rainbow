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
use rainbow_common::batch_requests::BatchRequests;
use rainbow_common::config::global_config::ApplicationGlobalConfig;
use serde::Deserialize;
use std::sync::Arc;

#[derive(Clone)]
pub struct PolicyTemplateEntityRouter {
    service: Arc<dyn PolicyTemplateEntityTrait>,
    config: Arc<ApplicationGlobalConfig>,
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

impl FromRef<PolicyTemplateEntityRouter> for Arc<ApplicationGlobalConfig> {
    fn from_ref(state: &PolicyTemplateEntityRouter) -> Self {
        state.config.clone()
    }
}

impl PolicyTemplateEntityRouter {
    pub fn new(service: Arc<dyn PolicyTemplateEntityTrait>, config: Arc<ApplicationGlobalConfig>) -> Self {
        Self { service, config }
    }

    pub fn router(self) -> Router {
        Router::new()
            .route("/", get(Self::handle_get_all_policy_templates))
            .route("/", post(Self::handle_create_policy_template))
            .route("/batch", post(Self::handle_get_batch_policy_templates))
            .route("/:id", get(Self::handle_get_policy_template_by_id))
            .route("/:id", delete(Self::handle_delete_policy_template_by_id))
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
        input: Result<Json<BatchRequests>, JsonRejection>,
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
        let id_urn = match parse_urn(&id) {
            Ok(urn) => urn,
            Err(resp) => return resp,
        };
        match state.service.get_policy_template_by_id(&id_urn).await {
            Ok(Some(template)) => (StatusCode::OK, Json(ToCamelCase(template))).into_response(),
            Ok(None) => (StatusCode::NOT_FOUND).into_response(),
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
    async fn handle_delete_policy_template_by_id(
        State(state): State<PolicyTemplateEntityRouter>,
        Path(id): Path<String>,
    ) -> impl IntoResponse {
        let id_urn = match parse_urn(&id) {
            Ok(urn) => urn,
            Err(resp) => return resp,
        };
        match state.service.delete_policy_template_by_id(&id_urn).await {
            Ok(_) => StatusCode::OK.into_response(),
            Err(err) => err.to_response(),
        }
    }
}
