#![allow(unused)]
use axum::extract::rejection::JsonRejection;
use axum::response::{IntoResponse, Response};
use axum::Json;
use rainbow_common::errors::helpers::BadFormat;
use rainbow_common::errors::{CommonErrors, ErrorLog};
use std::str::FromStr;
use tracing::error;
use urn::Urn;

pub(crate) fn extract_payload<T>(input: Result<Json<T>, JsonRejection>) -> Result<T, Response> {
    match input {
        Ok(Json(data)) => Ok(data),
        Err(err) => {
            let e = CommonErrors::format_new(BadFormat::Received, &format!("{}", err.body_text()));
            error!("{}", e.log());
            Err(e.into_response())
        }
    }
}

pub(crate) fn parse_urn(id: &str) -> Result<Urn, Response> {
    Urn::from_str(id).map_err(|err| {
        let e = CommonErrors::format_new(
            BadFormat::Received,
            &format!("Urn malformed: {}. Error: {}", id, err),
        );
        error!("{}", e.log());
        e.into_response()
    })
}
