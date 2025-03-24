use crate::consumer::core::ds_protocol::ds_protocol_err::DSProtocolTransferConsumerErrors;
use crate::consumer::core::ds_protocol::DSProtocolTransferConsumerTrait;
use axum::extract::rejection::JsonRejection;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};
use rainbow_common::err::transfer_err::TransferErrorType::{NotCheckedError, ProtocolBodyError};
use rainbow_common::protocol::transfer::transfer_completion::TransferCompletionMessage;
use rainbow_common::protocol::transfer::transfer_start::TransferStartMessage;
use rainbow_common::protocol::transfer::transfer_suspension::TransferSuspensionMessage;
use rainbow_common::protocol::transfer::transfer_termination::TransferTerminationMessage;
use rainbow_common::utils::get_urn_from_string;
use std::sync::Arc;
use tracing::info;

pub struct DSProtocolTransferConsumerRouter<T> {
    transfer_service: Arc<T>,
}

impl<T> DSProtocolTransferConsumerRouter<T>
where
    T: DSProtocolTransferConsumerTrait + Send + Sync + 'static,
{
    pub fn new(transfer_service: Arc<T>) -> Self {
        Self { transfer_service }
    }
    pub fn router(self) -> Router {
        Router::new()
            .route(
                "/:callback/transfers/:consumer_pid/start",
                post(Self::handle_transfer_start),
            )
            .route(
                "/:callback/transfers/:consumer_pid/suspension",
                post(Self::handle_transfer_suspension),
            )
            .route(
                "/:callback/transfers/:consumer_pid/completion",
                post(Self::handle_transfer_completion),
            )
            .route(
                "/:callback/transfers/:consumer_pid/termination",
                post(Self::handle_transfer_termination),
            )
            .with_state(self.transfer_service)
    }
    async fn handle_transfer_start(
        State(transfer_service): State<Arc<T>>,
        Path((callback, consumer_pid)): Path<(String, String)>,
        input: Result<Json<TransferStartMessage>, JsonRejection>,
    ) -> impl IntoResponse {
        info!("POST /{}/transfers/{}/start", callback, consumer_pid);
        let callback = match get_urn_from_string(&callback) {
            Ok(callback) => callback,
            Err(err) => return NotCheckedError { inner_error: err }.into_response(),
        };
        let consumer_pid = match get_urn_from_string(&consumer_pid) {
            Ok(consumer_pid) => consumer_pid,
            Err(err) => return NotCheckedError { inner_error: err }.into_response(),
        };
        let input = match input {
            Ok(input) => input.0,
            Err(e) => return ProtocolBodyError { message: e.body_text() }.into_response(),
        };

        match transfer_service.transfer_start(Some(callback), consumer_pid, input).await {
            Ok(transfer_process) => (StatusCode::OK, Json(transfer_process)).into_response(),
            Err(err) => match err.downcast::<DSProtocolTransferConsumerErrors>() {
                Ok(e) => e.into_response(),
                Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
            },
        }
    }
    async fn handle_transfer_suspension(
        State(transfer_service): State<Arc<T>>,
        Path((callback, consumer_pid)): Path<(String, String)>,
        input: Result<Json<TransferSuspensionMessage>, JsonRejection>,
    ) -> impl IntoResponse {
        info!("POST /{}/transfers/{}/suspension", callback, consumer_pid);
        let callback = match get_urn_from_string(&callback) {
            Ok(callback) => callback,
            Err(err) => return NotCheckedError { inner_error: err }.into_response(),
        };
        let consumer_pid = match get_urn_from_string(&consumer_pid) {
            Ok(consumer_pid) => consumer_pid,
            Err(err) => return NotCheckedError { inner_error: err }.into_response(),
        };
        let input = match input {
            Ok(input) => input.0,
            Err(e) => return ProtocolBodyError { message: e.body_text() }.into_response(),
        };

        match transfer_service.transfer_suspension(Some(callback), consumer_pid, input).await {
            Ok(transfer_process) => (StatusCode::OK, Json(transfer_process)).into_response(),
            Err(err) => match err.downcast::<DSProtocolTransferConsumerErrors>() {
                Ok(e) => e.into_response(),
                Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
            },
        }
    }
    async fn handle_transfer_completion(
        State(transfer_service): State<Arc<T>>,
        Path((callback, consumer_pid)): Path<(String, String)>,
        input: Result<Json<TransferCompletionMessage>, JsonRejection>,
    ) -> impl IntoResponse {
        info!("POST /{}/transfers/{}/start", callback, consumer_pid);
        let callback = match get_urn_from_string(&callback) {
            Ok(callback) => callback,
            Err(err) => return NotCheckedError { inner_error: err }.into_response(),
        };
        let consumer_pid = match get_urn_from_string(&consumer_pid) {
            Ok(consumer_pid) => consumer_pid,
            Err(err) => return NotCheckedError { inner_error: err }.into_response(),
        };
        let input = match input {
            Ok(input) => input.0,
            Err(e) => return ProtocolBodyError { message: e.body_text() }.into_response(),
        };

        match transfer_service.transfer_completion(Some(callback), consumer_pid, input).await {
            Ok(transfer_process) => (StatusCode::OK, Json(transfer_process)).into_response(),
            Err(err) => match err.downcast::<DSProtocolTransferConsumerErrors>() {
                Ok(e) => e.into_response(),
                Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
            },
        }
    }
    async fn handle_transfer_termination(
        State(transfer_service): State<Arc<T>>,
        Path((callback, consumer_pid)): Path<(String, String)>,
        input: Result<Json<TransferTerminationMessage>, JsonRejection>,
    ) -> impl IntoResponse {
        info!("POST /{}/transfers/{}/termination", callback, consumer_pid);
        let callback = match get_urn_from_string(&callback) {
            Ok(callback) => callback,
            Err(err) => return NotCheckedError { inner_error: err }.into_response(),
        };
        let consumer_pid = match get_urn_from_string(&consumer_pid) {
            Ok(consumer_pid) => consumer_pid,
            Err(err) => return NotCheckedError { inner_error: err }.into_response(),
        };
        let input = match input {
            Ok(input) => input.0,
            Err(e) => return ProtocolBodyError { message: e.body_text() }.into_response(),
        };

        match transfer_service.transfer_termination(Some(callback), consumer_pid, input).await {
            Ok(transfer_process) => (StatusCode::OK, Json(transfer_process)).into_response(),
            Err(err) => match err.downcast::<DSProtocolTransferConsumerErrors>() {
                Ok(e) => e.into_response(),
                Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
            },
        }
    }
}
