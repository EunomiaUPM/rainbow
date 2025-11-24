use crate::core::dsp::errors::error_adapter::DspTransferError;
use axum::extract::rejection::JsonRejection;
use axum::Json;
use rainbow_common::errors::helpers::BadFormat;
use rainbow_common::errors::{CommonErrors, ErrorLog};
use tracing::error;

pub(crate) mod error_adapter;

pub(crate) fn extract_payload_error<T>(input: Result<Json<T>, JsonRejection>) -> anyhow::Result<T, DspTransferError> {
    match input {
        Ok(Json(data)) => Ok(data),
        Err(err) => {
            let e = CommonErrors::format_new(BadFormat::Received, &format!("{}", err.body_text()));
            error!("{}", e.log());
            Err(e.into())
        }
    }
}
