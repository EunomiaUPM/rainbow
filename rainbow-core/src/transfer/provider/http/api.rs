use crate::transfer::provider::lib::api::get_all_transfers;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};

// TODO CONTINUE HERE
pub fn router() -> Router {
    Router::new().route("/api/v1/transfers", get(handle_get_all_transfers))
}

async fn handle_get_all_transfers() -> impl IntoResponse {
    let transfers = get_all_transfers().await;
    if transfers.is_err() {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }
    (StatusCode::OK, Json(transfers.unwrap())).into_response()
}
