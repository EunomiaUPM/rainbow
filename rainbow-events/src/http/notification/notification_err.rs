use crate::core::notification::notification_err::NotificationErrors;
use crate::core::subscription::subscription_err::SubscriptionErrorMessage;
use axum::response::{IntoResponse, Response};
use axum::Json;
use reqwest::StatusCode;

impl IntoResponse for NotificationErrors {
    fn into_response(self) -> Response {
        match self {
            e @ NotificationErrors::DbErr(..) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(SubscriptionErrorMessage {
                    code: "500".to_string(),
                    title: "INTERNAL_ERROR".to_string(),
                    message: e.to_string(),
                }),
            ).into_response(),
            e @ NotificationErrors::NotFound { .. } => (
                StatusCode::NOT_FOUND,
                Json(SubscriptionErrorMessage {
                    code: "404".to_string(),
                    title: "NOT_FOUND".to_string(),
                    message: e.to_string(),
                }),
            ).into_response(),
            e @ NotificationErrors::JsonRejection { .. } => (
                StatusCode::BAD_REQUEST,
                Json(SubscriptionErrorMessage {
                    code: "400".to_string(),
                    title: "JSON_MALFORMED".to_string(),
                    message: e.to_string(),
                }),
            ).into_response(),
            e @ NotificationErrors::UrnUuidSchema { .. } => (
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
