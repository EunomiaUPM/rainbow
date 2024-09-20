use crate::transfer::protocol::messages::TransferKickOff;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};

pub fn router() -> Router {
    Router::new().route("/transfers", post(handle_kickoff_transfer))
}

async fn handle_kickoff_transfer(Json(input): Json<TransferKickOff>) -> impl IntoResponse {
    // it must create consumer_id
    // it must create callback_address and persist it
    // it should create TransferRequestMessage and send it to dataspace
    // it should have the url of the provider
    (StatusCode::OK)
}
