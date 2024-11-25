use crate::common::http::client::DATA_PLANE_HTTP_CLIENT;
use crate::provider::data::entities::transfer_process;
use crate::provider::lib::data_plane::resolve_endpoint_from_agreement;
use axum::body::Body;
use axum::extract::Path;
use axum::http::{Response, StatusCode};
use axum::response::IntoResponse;
use axum::routing::{get, post, put};
use axum::{Json, Router};
use rainbow_common::config::database::get_db_connection;
use rainbow_common::protocol::transfer::TransferStateForDb;
use sea_orm::ColumnTrait;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;
use tracing::{error, info};
use uuid::Uuid;

pub fn router() -> Router {
    Router::new().route("/data/pull/:data_id", get(handle_data_pull_proxy))
        .route("/data/push/:data_id", post(handle_data_push_proxy))
        .route("/data/push/:data_id", put(handle_data_push_proxy))
}


async fn handle_data_pull_proxy(Path(data_id): Path<Uuid>) -> impl IntoResponse {
    info!("Forwarding data from provider data plane proxy to dataspace");
    info!("GET /data/pull/{}", data_id);
    let db_connection = get_db_connection().await;

    // Resolve transfer process and endpoint...
    let transfer_process_from_db = transfer_process::Entity::find()
        .filter(transfer_process::Column::DataPlaneId.eq(data_id))
        .one(db_connection)
        .await
        .unwrap();

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
    let res = match DATA_PLANE_HTTP_CLIENT.get(endpoint.unwrap().dcat.endpoint_url).send().await {
        Ok(res) => res,
        Err(err) => {
            error!("Error sending request: {:?}", err);
            return (
                StatusCode::BAD_GATEWAY,
                "Upstream server error. Endpoint not available".to_string(),
            )
                .into_response();
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

async fn handle_data_push_proxy(
    Path(data_id): Path<Uuid>,
    Json(input): Json<serde_json::Value>,
) -> impl IntoResponse {
    info!("Forwarding data from provider data plane proxy to consumer");
    info!("POST-PUT /data/push/{}", data_id);

    let db_connection = get_db_connection().await;

    // Resolve transfer process and endpoint...
    let transfer_process_from_db = transfer_process::Entity::find()
        .filter(transfer_process::Column::DataPlaneId.eq(data_id))
        .one(db_connection)
        .await
        .unwrap();

    if transfer_process_from_db.is_none() {
        return (StatusCode::NOT_FOUND, "Not found".to_string()).into_response();
    }

    let transfer_process = transfer_process_from_db.unwrap();

    if transfer_process.state != TransferStateForDb::STARTED {
        return (StatusCode::UNAUTHORIZED, "Not authorized".to_string()).into_response();
    }


    let endpoint = "asdas";
    // let endpoint_url = endpoint.get("dspace:endpoint").and_then(|v| v.as_str()).unwrap();


    // Send the request to the downstream server
    let res = match DATA_PLANE_HTTP_CLIENT.post(endpoint).json(&input).send().await {
        Ok(res) => res,
        Err(err) => {
            error!("Error sending request: {:?}", err);
            return (
                StatusCode::BAD_GATEWAY,
                "Upstream server error. Endpoint not available".to_string(),
            )
                .into_response();
        }
    };

    res.status().into_response()
}