use axum::{
    extract::{rejection::JsonRejection, FromRef, State},
    http::StatusCode,
    response::IntoResponse,
    routing::post,
    Json, Router,
};
use serde::Serialize;
use std::future::Future;
use std::sync::Arc;

use crate::protocols::dsp::errors::extract_payload_error;
use crate::protocols::dsp::orchestrator::rpc::types::{
    RpcTransferCompletionMessageDto, RpcTransferErrorDto, RpcTransferRequestMessageDto, RpcTransferStartMessageDto,
    RpcTransferSuspensionMessageDto, RpcTransferTerminationMessageDto,
};
use crate::protocols::dsp::orchestrator::OrchestratorTrait;
use crate::protocols::dsp::protocol_types::{
    TransferErrorDto, TransferProcessMessageType, TransferProcessMessageWrapper,
};
use rainbow_common::config::services::TransferConfig;
use rainbow_common::errors::CommonErrors;
use rainbow_common::protocol::context_field::ContextField;

#[derive(Clone)]
pub struct RpcRouter {
    orchestrator: Arc<dyn OrchestratorTrait>,
    config: Arc<TransferConfig>,
}

impl FromRef<RpcRouter> for Arc<dyn OrchestratorTrait> {
    fn from_ref(state: &RpcRouter) -> Self {
        state.orchestrator.clone()
    }
}

impl FromRef<RpcRouter> for Arc<TransferConfig> {
    fn from_ref(state: &RpcRouter) -> Self {
        state.config.clone()
    }
}

impl RpcRouter {
    pub fn new(service: Arc<dyn OrchestratorTrait>, config: Arc<TransferConfig>) -> Self {
        Self { orchestrator: service, config }
    }

    pub fn router(self) -> Router {
        Router::new()
            .route(
                "/rpc/setup-request",
                post(Self::handle_transfer_request_rpc),
            )
            .route("/rpc/setup-start", post(Self::handle_transfer_start_rpc))
            .route(
                "/rpc/setup-completion",
                post(Self::handle_transfer_completion_rpc),
            )
            .route(
                "/rpc/setup-termination",
                post(Self::handle_transfer_termination_rpc),
            )
            .route(
                "/rpc/setup-suspension",
                post(Self::handle_transfer_suspension_rpc),
            )
            .with_state(self)
    }

    async fn process_request<T, R, F, Fut>(
        input: Result<Json<T>, JsonRejection>,
        success_code: StatusCode,
        action: F,
    ) -> impl IntoResponse
    where
        T: Send + Serialize + Clone + 'static,
        R: Serialize,
        F: FnOnce(T) -> Fut,
        Fut: Future<Output = anyhow::Result<R>> + Send,
    {
        let payload = match extract_payload_error(input) {
            Ok(v) => v,
            Err(e) => {
                let error_dto: TransferProcessMessageWrapper<TransferErrorDto> = e.into();
                return (StatusCode::BAD_REQUEST, Json(error_dto)).into_response();
            }
        };
        Self::map_service_result(action(payload.clone()).await, success_code, payload).into_response()
    }

    fn map_service_result<R, T>(
        result: anyhow::Result<R>,
        success_code: StatusCode,
        original_request: T,
    ) -> impl IntoResponse
    where
        R: Serialize,
        T: Serialize + Clone,
    {
        match result {
            Ok(data) => (success_code, Json(data)).into_response(),
            Err(err) => {
                let error_wrapper: TransferProcessMessageWrapper<TransferErrorDto> =
                    match err.downcast::<CommonErrors>() {
                        Ok(common_errors) => common_errors.into(),
                        Err(original_err) => TransferProcessMessageWrapper {
                            context: ContextField::default(),
                            _type: TransferProcessMessageType::TransferError,
                            dto: TransferErrorDto {
                                consumer_pid: None,
                                provider_pid: None,
                                code: Some("5000".to_string()),
                                reason: Some(vec![original_err.to_string()]),
                            },
                        },
                    };
                let rpc_error_dto: RpcTransferErrorDto<T> =
                    RpcTransferErrorDto { request: original_request, error: error_wrapper };

                (StatusCode::BAD_REQUEST, Json(rpc_error_dto)).into_response()
            }
        }
    }

    async fn handle_transfer_request_rpc(
        State(state): State<RpcRouter>,
        input: Result<Json<RpcTransferRequestMessageDto>, JsonRejection>,
    ) -> impl IntoResponse {
        Self::process_request(input, StatusCode::CREATED, |data| async move {
            state.orchestrator.get_rpc_service().setup_transfer_request(&data).await
        })
        .await
    }

    async fn handle_transfer_start_rpc(
        State(state): State<RpcRouter>,
        input: Result<Json<RpcTransferStartMessageDto>, JsonRejection>,
    ) -> impl IntoResponse {
        Self::process_request(input, StatusCode::ACCEPTED, |data| async move {
            state.orchestrator.get_rpc_service().setup_transfer_start(&data).await
        })
        .await
    }

    async fn handle_transfer_completion_rpc(
        State(state): State<RpcRouter>,
        input: Result<Json<RpcTransferCompletionMessageDto>, JsonRejection>,
    ) -> impl IntoResponse {
        Self::process_request(input, StatusCode::ACCEPTED, |data| async move {
            state.orchestrator.get_rpc_service().setup_transfer_completion(&data).await
        })
        .await
    }

    async fn handle_transfer_termination_rpc(
        State(state): State<RpcRouter>,
        input: Result<Json<RpcTransferTerminationMessageDto>, JsonRejection>,
    ) -> impl IntoResponse {
        Self::process_request(input, StatusCode::ACCEPTED, |data| async move {
            state.orchestrator.get_rpc_service().setup_transfer_termination(&data).await
        })
        .await
    }

    async fn handle_transfer_suspension_rpc(
        State(state): State<RpcRouter>,
        input: Result<Json<RpcTransferSuspensionMessageDto>, JsonRejection>,
    ) -> impl IntoResponse {
        Self::process_request(input, StatusCode::ACCEPTED, |data| async move {
            state.orchestrator.get_rpc_service().setup_transfer_suspension(&data).await
        })
        .await
    }
}
