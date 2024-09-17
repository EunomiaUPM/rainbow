use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use uuid::Uuid;

pub fn router() -> Router
{
    Router::new()
        .route("/transfer/data/:transfer_id", get(handle_pull_data))
}

async fn handle_pull_data(Path(transfer_id): Path<Uuid>) -> impl IntoResponse {
    (StatusCode::OK, transfer_id.to_string()).into_response()
}

