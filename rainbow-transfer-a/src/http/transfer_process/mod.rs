use std::str::FromStr;
use crate::entities::transfer_process::{EditTransferProcessDto, NewTransferProcessDto, TransferAgentProcessesTrait};
use crate::errors::error_adapter::CustomToResponse;
use axum::extract::rejection::JsonRejection;
use axum::extract::{FromRef, Path, Query, Request, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::{delete, get, post, put};
use axum::{Json, Router};
use rainbow_common::batch_requests::BatchRequests;
use rainbow_common::config::provider_config::ApplicationProviderConfig;
use rainbow_common::errors::helpers::BadFormat;
use rainbow_common::errors::{CommonErrors, ErrorLog};
use rainbow_common::utils::get_urn_from_string;
use serde::Deserialize;
use std::sync::Arc;
use tower_http::trace::{DefaultOnResponse, TraceLayer};
use tracing::{error, info, Level, Span};
use urn::Urn;
use uuid::Uuid;

#[derive(Clone)]
pub struct TransferAgentProcessesRouter {
    service: Arc<dyn TransferAgentProcessesTrait>,
    config: Arc<ApplicationProviderConfig>,
}

#[derive(Deserialize)]
pub struct PaginationParams {
    pub limit: Option<u64>,
    pub page: Option<u64>,
}

impl FromRef<TransferAgentProcessesRouter> for Arc<dyn TransferAgentProcessesTrait> {
    fn from_ref(state: &TransferAgentProcessesRouter) -> Self {
        state.service.clone()
    }
}

impl FromRef<TransferAgentProcessesRouter> for Arc<ApplicationProviderConfig> {
    fn from_ref(state: &TransferAgentProcessesRouter) -> Self {
        state.config.clone()
    }
}

impl TransferAgentProcessesRouter {
    pub fn new(service: Arc<dyn TransferAgentProcessesTrait>, config: Arc<ApplicationProviderConfig>) -> Self {
        Self { service, config }
    }

    pub fn router(self) -> Router {
        Router::new()
            .route(
                "/transfer-processes",
                get(Self::handle_get_all_processes).post(Self::handle_create_process),
            )
            .route("/transfer-processes/batch", post(Self::handle_get_batch_processes))
            .route(
                "/transfer-processes/:id",
                get(Self::handle_get_process_by_id)
                    .put(Self::handle_put_process)
                    .delete(Self::handle_delete_process),
            )
            .route(
                "/transfer-processes/:id/key/:key_id",
                get(Self::handle_get_process_by_key_id),
            )
            .with_state(self)
    }

    fn parse_urn(id: &str) -> Result<Urn, Response> {
        Urn::from_str(id).map_err(|err| {
            let e = CommonErrors::format_new(
                BadFormat::Received,
                &format!("Urn malformed: {}. Error: {}", id, err)
            );
            error!("{}", e.log());
            e.into_response()
        })
    }

    fn extract_payload<T>(input: Result<Json<T>, JsonRejection>) -> Result<T, Response> {
        match input {
            Ok(Json(data)) => Ok(data),
            Err(err) => {
                let e = CommonErrors::format_new(BadFormat::Received, &format!("{}", err.body_text()));
                error!("{}", e.log());
                Err(e.into_response())
            }
        }
    }

    async fn handle_get_all_processes(
        State(state): State<TransferAgentProcessesRouter>,
        Query(params): Query<PaginationParams>,
    ) -> impl IntoResponse {
        match state.service.get_all_transfer_processes(params.limit, params.page).await {
            Ok(processes) => (StatusCode::OK, Json(processes)).into_response(),
            Err(err) => err.to_response(),
        }
    }

    async fn handle_create_process(
        State(state): State<TransferAgentProcessesRouter>,
        input: Result<Json<NewTransferProcessDto>, JsonRejection>,
    ) -> impl IntoResponse {
        let input = match Self::extract_payload(input) {
            Ok(v) => v,
            Err(e) => return e,
        };
        match state.service.create_transfer_process(&input).await {
            Ok(created_process) => (StatusCode::CREATED, Json(created_process)).into_response(),
            Err(err) => err.to_response(),
        }
    }

    async fn handle_get_batch_processes(
        State(state): State<TransferAgentProcessesRouter>,
        input: Result<Json<BatchRequests>, JsonRejection>,
    ) -> impl IntoResponse {
        let input = match Self::extract_payload(input) {
            Ok(v) => v,
            Err(e) => return e,
        };
        match state.service.get_batch_transfer_processes(&input.ids).await {
            Ok(processes) => (StatusCode::OK, Json(processes)).into_response(),
            Err(err) => err.to_response(),
        }
    }

    async fn handle_get_process_by_id(
        State(state): State<TransferAgentProcessesRouter>,
        Path(id): Path<String>,
    ) -> impl IntoResponse {
        let id_urn = match Self::parse_urn(&id) {
            Ok(urn) => urn,
            Err(resp) => return resp,
        };
        match state.service.get_transfer_process_by_id(&id_urn).await {
            Ok(process) => (StatusCode::OK, Json(process)).into_response(),
            Err(err) => err.to_response(),
        }
    }

    async fn handle_put_process(
        State(state): State<TransferAgentProcessesRouter>,
        Path(id): Path<String>,
        input: Result<Json<EditTransferProcessDto>, JsonRejection>,
    ) -> impl IntoResponse {
        let id_urn = match Self::parse_urn(&id) {
            Ok(urn) => urn,
            Err(resp) => return resp,
        };
        let input = match Self::extract_payload(input) {
            Ok(v) => v,
            Err(e) => return e,
        };
        match state.service.put_transfer_process(&id_urn, &input).await {
            Ok(updated_process) => (StatusCode::OK, Json(updated_process)).into_response(),
            Err(err) => err.to_response(),
        }
    }

    async fn handle_delete_process(
        State(state): State<TransferAgentProcessesRouter>,
        Path(id): Path<String>,
    ) -> impl IntoResponse {
        let id_urn = match Self::parse_urn(&id) {
            Ok(urn) => urn,
            Err(resp) => return resp,
        };
        match state.service.delete_transfer_process(&id_urn).await {
            Ok(_) => (StatusCode::NO_CONTENT).into_response(),
            Err(err) => err.to_response(),
        }
    }

    async fn handle_get_process_by_key_id(
        State(state): State<TransferAgentProcessesRouter>,
        Path((id, key_id)): Path<(String, String)>,
    ) -> impl IntoResponse {
        let id_urn = match Self::parse_urn(&id) {
            Ok(urn) => urn,
            Err(resp) => return resp,
        };
        match state.service.get_transfer_process_by_key_id(&key_id, &id_urn).await {
            Ok(process) => (StatusCode::OK, Json(process)).into_response(),
            Err(err) => err.to_response(),
        }
    }
}