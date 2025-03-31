use crate::core::subscription::subscription_err::{SubscriptionErrorMessage, SubscriptionErrors};
use axum::response::{IntoResponse, Response};
use axum::Json;
use reqwest::StatusCode;

impl IntoResponse for SubscriptionErrors {
    fn into_response(self) -> Response {
        match self {
            e @ SubscriptionErrors::DbErr(..) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(SubscriptionErrorMessage {
                    code: "500".to_string(),
                    title: "INTERNAL_ERROR".to_string(),
                    message: e.to_string(),
                }),
            ).into_response(),
            e @ SubscriptionErrors::NotFound { .. } => (
                StatusCode::NOT_FOUND,
                Json(SubscriptionErrorMessage {
                    code: "404".to_string(),
                    title: "NOT_FOUND".to_string(),
                    message: e.to_string(),
                }),
            ).into_response(),
            e @ SubscriptionErrors::JsonRejection { .. } => (
                StatusCode::BAD_REQUEST,
                Json(SubscriptionErrorMessage {
                    code: "400".to_string(),
                    title: "JSON_MALFORMED".to_string(),
                    message: e.to_string(),
                }),
            ).into_response(),
            e @ SubscriptionErrors::UrnUuidSchema { .. } => (
                StatusCode::BAD_REQUEST,
                Json(SubscriptionErrorMessage {
                    code: "400".to_string(),
                    title: "UUID_SCHEMA".to_string(),
                    message: e.to_string(),
                }),
            ).into_response()
        }
    }
}
