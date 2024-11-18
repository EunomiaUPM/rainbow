use crate::common::http::client::DATA_PLANE_HTTP_CLIENT;
use crate::protocol::messages::TransferStateForDb;
use crate::provider::data::entities::transfer_process;
use crate::provider::lib::data_plane::resolve_endpoint_from_agreement;
use rainbow_common::config::database::get_db_connection;
use axum::body::Body;
use axum::extract::Path;
use axum::http::{Response, StatusCode};
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use sea_orm::ColumnTrait;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;
use tracing::{error, info};
use uuid::Uuid;

pub fn router() -> Router {
    Router::new().route("/data/:data_id", get(handle_data_proxy))
}

async fn handle_data_proxy(Path(data_id): Path<Uuid>) -> impl IntoResponse {
    info!("Forwarding data from provider data plane proxy");
    info!("GET /data/{}", data_id);
    let db_connection = get_db_connection().await;

    // Resolve transfer process and endpoint...
    let transfer_process_from_db = transfer_process::Entity::find()
        .filter(transfer_process::Column::DataPlaneId.eq(data_id))
        .one(db_connection)
        .await.unwrap();

    // let transfer_process_from_db =
    //     TRANSFER_PROVIDER_REPO.get_transfer_process_by_data_plane_process(data_id).unwrap();

    if transfer_process_from_db.is_none() {
        return (StatusCode::NOT_FOUND, "Not found".to_string()).into_response();
    }

    let transfer_process = transfer_process_from_db.unwrap();

    if transfer_process.state != TransferStateForDb::STARTED {
        return (StatusCode::UNAUTHORIZED, "Not authorized".to_string()).into_response();
    }

    let endpoint = resolve_endpoint_from_agreement(transfer_process.agreement_id).await;

    if endpoint.is_err() {
        return (
            StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS,
            "Not available".to_string(),
        )
            .into_response();
    }

    // Send the request to the upstream server
    let res = match DATA_PLANE_HTTP_CLIENT.get(endpoint.unwrap()).send().await {
        Ok(res) => res,
        Err(err) => {
            error!("Error sending request: {:?}", err);
            return (StatusCode::BAD_GATEWAY, "Upstream server error".to_string()).into_response();
        }
    };

    // Extract status, headers, and body
    let status = res.status();
    let headers = res.headers().clone();
    let body_bytes = match res.bytes().await {
        Ok(bytes) => bytes,
        Err(err) => {
            error!("Error reading response body: {:?}", err);
            return (
                StatusCode::BAD_GATEWAY,
                "Error reading upstream response".to_string(),
            )
                .into_response();
        }
    };

    // Build a new response including the headers
    let mut response =
        Response::builder().status(status).body(Body::from(body_bytes)).unwrap_or_else(|err| {
            error!("Error building response: {:?}", err);
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from("Internal server error"))
                .unwrap()
        });

    // Copy headers from the upstream response
    *response.headers_mut() = headers;

    response
}
