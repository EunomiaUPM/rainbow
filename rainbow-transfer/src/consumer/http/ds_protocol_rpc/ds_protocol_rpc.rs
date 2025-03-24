use crate::consumer::core::ds_protocol::ds_protocol_err::DSProtocolTransferConsumerErrors;
use crate::consumer::core::ds_protocol_rpc::ds_protocol_rpc_err::DSRPCTransferConsumerErrors;
use crate::consumer::core::ds_protocol_rpc::ds_protocol_rpc_types::{DSRPCTransferConsumerCompletionRequest, DSRPCTransferConsumerRequestRequest, DSRPCTransferConsumerStartRequest, DSRPCTransferConsumerSuspensionRequest, DSRPCTransferConsumerTerminationRequest};
use crate::consumer::core::ds_protocol_rpc::DSRPCTransferConsumerTrait;
use axum::extract::rejection::JsonRejection;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};
use log::info;
use rainbow_common::err::transfer_err::TransferErrorType::NotCheckedError;
use reqwest::StatusCode;
use std::sync::Arc;

pub struct DSRPCTransferConsumerRouter<T> {
    transfer_rpc_service: Arc<T>,
}

impl<T> DSRPCTransferConsumerRouter<T>
where
    T: DSRPCTransferConsumerTrait + Send + Sync + 'static,
{
    pub fn new(transfer_rpc_service: Arc<T>) -> Self {
        Self { transfer_rpc_service }
    }
    pub fn router(self) -> Router {
        Router::new()
            .route("/api/v1/transfers/rpc/setup-request", post(Self::setup_request))
            .route("/api/v1/transfers/rpc/setup-start", post(Self::setup_start))
            .route("/api/v1/transfers/rpc/setup-suspension", post(Self::setup_suspension))
            .route("/api/v1/transfers/rpc/setup-completion", post(Self::setup_completion))
            .route("/api/v1/transfers/rpc/setup-termination", post(Self::setup_termination))
            .with_state(self.transfer_rpc_service)
    }
    async fn setup_request(
        State(transfer_rpc_service): State<Arc<T>>,
        input: Result<Json<DSRPCTransferConsumerRequestRequest>, JsonRejection>,
    ) -> impl IntoResponse {
        info!("POST /api/v1/transfers/rpc/setup-request");
        let input = match input {
            Ok(input) => input.0,
            Err(e) => return DSProtocolTransferConsumerErrors::JsonRejection(e.to_string()).into_response(),
        };
        match transfer_rpc_service.setup_request(input).await {
            Ok(res) => (StatusCode::OK, Json(res)).into_response(),
            Err(e) => match e.downcast::<DSRPCTransferConsumerErrors>() {
                Ok(res_) => res_.into_response(),
                Err(e__) => (StatusCode::INTERNAL_SERVER_ERROR, NotCheckedError {
                    inner_error: e__
                }).into_response()
            }
        }
    }
    async fn setup_start(
        State(transfer_rpc_service): State<Arc<T>>,
        input: Result<Json<DSRPCTransferConsumerStartRequest>, JsonRejection>,
    ) -> impl IntoResponse {
        info!("POST /api/v1/transfers/rpc/setup-start");
        let input = match input {
            Ok(input) => input.0,
            Err(e) => return DSProtocolTransferConsumerErrors::JsonRejection(e.to_string()).into_response(),
        };
        match transfer_rpc_service.setup_start(input).await {
            Ok(res) => (StatusCode::OK, Json(res)).into_response(),
            Err(e) => match e.downcast::<DSRPCTransferConsumerErrors>() {
                Ok(res_) => res_.into_response(),
                Err(e__) => (StatusCode::INTERNAL_SERVER_ERROR, NotCheckedError {
                    inner_error: e__
                }).into_response()
            }
        }
    }
    async fn setup_suspension(
        State(transfer_rpc_service): State<Arc<T>>,
        input: Result<Json<DSRPCTransferConsumerSuspensionRequest>, JsonRejection>,
    ) -> impl IntoResponse {
        info!("POST /api/v1/transfers/rpc/setup-suspension");
        let input = match input {
            Ok(input) => input.0,
            Err(e) => return DSProtocolTransferConsumerErrors::JsonRejection(e.to_string()).into_response(),
        };
        match transfer_rpc_service.setup_suspension(input).await {
            Ok(res) => (StatusCode::OK, Json(res)).into_response(),
            Err(e) => match e.downcast::<DSRPCTransferConsumerErrors>() {
                Ok(res_) => res_.into_response(),
                Err(e__) => (StatusCode::INTERNAL_SERVER_ERROR, NotCheckedError {
                    inner_error: e__
                }).into_response()
            }
        }
    }
    async fn setup_completion(
        State(transfer_rpc_service): State<Arc<T>>,
        input: Result<Json<DSRPCTransferConsumerCompletionRequest>, JsonRejection>,
    ) -> impl IntoResponse {
        info!("POST /api/v1/transfers/rpc/setup-completion");
        let input = match input {
            Ok(input) => input.0,
            Err(e) => return DSProtocolTransferConsumerErrors::JsonRejection(e.to_string()).into_response(),
        };
        match transfer_rpc_service.setup_completion(input).await {
            Ok(res) => (StatusCode::OK, Json(res)).into_response(),
            Err(e) => match e.downcast::<DSRPCTransferConsumerErrors>() {
                Ok(res_) => res_.into_response(),
                Err(e__) => (StatusCode::INTERNAL_SERVER_ERROR, NotCheckedError {
                    inner_error: e__
                }).into_response()
            }
        }
    }
    async fn setup_termination(
        State(transfer_rpc_service): State<Arc<T>>,
        input: Result<Json<DSRPCTransferConsumerTerminationRequest>, JsonRejection>,
    ) -> impl IntoResponse {
        info!("POST /api/v1/transfers/rpc/setup-termination");
        let input = match input {
            Ok(input) => input.0,
            Err(e) => return DSProtocolTransferConsumerErrors::JsonRejection(e.to_string()).into_response(),
        };
        match transfer_rpc_service.setup_termination(input).await {
            Ok(res) => (StatusCode::OK, Json(res)).into_response(),
            Err(e) => match e.downcast::<DSRPCTransferConsumerErrors>() {
                Ok(res_) => res_.into_response(),
                Err(e__) => (StatusCode::INTERNAL_SERVER_ERROR, NotCheckedError {
                    inner_error: e__
                }).into_response()
            }
        }
    }
}