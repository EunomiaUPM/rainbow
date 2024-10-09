use crate::fake_catalog::lib::get_dataset_by_id;
use crate::fake_catalog::lib::get_datasets_by_endpoint;
use crate::fake_contracts::lib::get_agreement_by_id;
use crate::transfer::provider::data::repo::get_data_plane_process_by_id;
use crate::transfer::provider::http::client::DATA_PLANE_HTTP_CLIENT;
use crate::transfer::provider::lib::data_plane::resolve_endpoint_from_agreement;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use tracing::info;
use uuid::Uuid;

pub fn router() -> Router {
    Router::new().route("/data/:data_id", get(handle_data_proxy))
}

async fn handle_data_proxy(Path(data_id): Path<Uuid>) -> impl IntoResponse {
    info!("GET /data/{}", data_id.to_string());

    // Resolve consumer, provider, agreement, endpoint, status...
    // Resolve data plane process
    let data_plane_process = get_data_plane_process_by_id(data_id).unwrap();
    if data_plane_process.is_none() {
        return (StatusCode::NOT_FOUND, "Not found".to_string());
    }

    let data_plane_state = data_plane_process.as_ref().unwrap().state;
    if data_plane_state == false {
        return (StatusCode::UNAUTHORIZED, "Not found".to_string());
    }

    let endpoint = resolve_endpoint_from_agreement(data_plane_process.unwrap().agreement_id).await;
    if endpoint.is_err() {
        return (
            StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS,
            "Not available".to_string(),
        );
    }

    // TODO create match with QUIC, KAFKA...
    let res = DATA_PLANE_HTTP_CLIENT
        .get(&endpoint.unwrap())
        .send()
        .await
        .unwrap();

    // Forward request
    (StatusCode::OK, res.text().await.unwrap())
}
