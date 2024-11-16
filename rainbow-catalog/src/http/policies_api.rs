use axum::extract::Path;
use axum::response::IntoResponse;
use axum::routing::{delete, get, post};
use axum::{Json, Router};
use log::info;
use reqwest::StatusCode;
use serde_json::Value;
use tower_http::trace::TraceLayer;
use uuid::Uuid;
use crate::core::policies_api::{delete_catalog_policies, delete_dataservices_policies, delete_dataset_policies, delete_distributions_policies, get_catalog_policies, get_dataservices_policies, get_dataset_policies, get_distributions_policies, post_catalog_policies, post_dataservices_policies, post_dataset_policies, post_distributions_policies};

pub async fn catalog_policies_api_router() -> anyhow::Result<Router> {
    let router = Router::new()
        .route("/api/v1/catalogs/:id/policies", get(handle_get_catalog_policies))
        .route("/api/v1/catalogs/:id/policies", post(handle_post_catalog_policies))
        .route("/api/v1/catalogs/:id/policies/:pid", delete(handle_delete_catalog_policies))

        .route("/api/v1/datasets/:id/policies", get(handle_get_dataset_policies))
        .route("/api/v1/datasets/:id/policies", post(handle_post_dataset_policies))
        .route("/api/v1/datasets/:id/policies/:pid", delete(handle_delete_dataset_policies))

        .route("/api/v1/data-services/:id/policies", get(handle_get_dataservices_policies))
        .route("/api/v1/data-services/:id/policies", post(handle_post_dataservices_policies))
        .route("/api/v1/data-services/:id/policies/:pid", delete(handle_delete_dataservices_policies))

        .route("/api/v1/distributions/:id/policies", get(handle_get_distributions_policies))
        .route("/api/v1/distributions/:id/policies", post(handle_post_distributions_policies))
        .route("/api/v1/distributions/:id/policies/:pid", delete(handle_delete_distributions_policies))

        .layer(TraceLayer::new_for_http());;
    Ok(router)
}

// here - do post and delete routes and commit

async fn handle_get_catalog_policies(
    Path(catalog_id): Path<Uuid>,
) -> impl IntoResponse {
    info!("GET /api/v1/catalogs/{}/policies", catalog_id.to_string());
    
    match get_catalog_policies(catalog_id).await {
        Ok(d) => (StatusCode::OK, Json(d)).into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response()
    }
}
async fn handle_post_catalog_policies(
    Path(catalog_id): Path<Uuid>,
    Json(input): Json<Value>, // TODO Odrl structure
) -> impl IntoResponse {
    info!("POST /api/v1/catalogs/{}/policies", catalog_id.to_string());
    
    match post_catalog_policies(catalog_id, input).await {
        Ok(d) => (StatusCode::CREATED, Json(d)).into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response()
    }
}
async fn handle_delete_catalog_policies(
    Path((catalog_id, policy_id)): Path<(Uuid, Uuid)>,
) -> impl IntoResponse {
    info!("DELETE /api/v1/catalogs/{}/policies/{}", catalog_id.to_string(), policy_id.to_string());
    
    match delete_catalog_policies(catalog_id, policy_id).await {
        Ok(d) => (StatusCode::ACCEPTED).into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response()
    }
}
async fn handle_get_dataset_policies(
    Path(dataset_id): Path<Uuid>,
) -> impl IntoResponse {
    info!("GET /api/v1/datasets/{}/policies", dataset_id.to_string());

    match get_dataset_policies(dataset_id).await {
        Ok(d) => (StatusCode::OK, Json(d)).into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response()
    }
}
async fn handle_post_dataset_policies(
    Path(dataset_id): Path<Uuid>,
    Json(input): Json<Value>,
) -> impl IntoResponse {
    info!("POST /api/v1/datasets/{}/policies", dataset_id.to_string());

    match post_dataset_policies(dataset_id, input).await {
        Ok(d) => (StatusCode::CREATED, Json(d)).into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response()
    }
}
async fn handle_delete_dataset_policies(
    Path((dataset_id, policy_id)): Path<(Uuid, Uuid)>,
) -> impl IntoResponse {
    info!("DELETE /api/v1/datasets/{}/policies/{}", dataset_id.to_string(), policy_id.to_string());

    match delete_dataset_policies(dataset_id, policy_id).await {
        Ok(d) => (StatusCode::ACCEPTED).into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response()
    }
}
async fn handle_get_dataservices_policies(
    Path(dataservice_id): Path<Uuid>,
) -> impl IntoResponse {
    info!("GET /api/v1/data-services/{}/policies", dataservice_id.to_string());
    
    match get_dataservices_policies(dataservice_id).await {
        Ok(d) => (StatusCode::OK, Json(d)).into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response()
    }
}
async fn handle_post_dataservices_policies(
    Path(dataservice_id): Path<Uuid>,
    Json(input): Json<Value>,
) -> impl IntoResponse {
    info!("POST /api/v1/data-services/{}/policies", dataservice_id.to_string());

    match post_dataservices_policies(dataservice_id, input).await {
        Ok(d) => (StatusCode::CREATED, Json(d)).into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response()
    }
}
async fn handle_delete_dataservices_policies(
    Path((dataservice_id, policy_id)): Path<(Uuid, Uuid)>,
) -> impl IntoResponse {
    info!("DELETE /api/v1/data-services/{}/policies/{}", dataservice_id.to_string(), policy_id.to_string());

    match delete_dataservices_policies(dataservice_id, policy_id).await {
        Ok(d) => (StatusCode::ACCEPTED).into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response()
    }
}
async fn handle_get_distributions_policies(
    Path(distribution_id): Path<Uuid>,
) -> impl IntoResponse {
    info!("GET /api/v1/distributions/{}/policies", distribution_id.to_string());
    
    match get_distributions_policies(distribution_id).await {
        Ok(d) => (StatusCode::OK, Json(d)).into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response()
    }
}
async fn handle_post_distributions_policies(
    Path(distribution_id): Path<Uuid>,
    Json(input): Json<Value>,
) -> impl IntoResponse {
    info!("POST /api/v1/distributions/{}/policies", distribution_id.to_string());

    match post_distributions_policies(distribution_id, input).await {
        Ok(d) => (StatusCode::CREATED, Json(d)).into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response()
    }
}
async fn handle_delete_distributions_policies(
    Path((distribution_id, policy_id)): Path<(Uuid, Uuid)>,
) -> impl IntoResponse {
    info!("DELETE /api/v1/distributions/{}/policies/{}", distribution_id.to_string(), policy_id.to_string());

    match delete_distributions_policies(distribution_id, policy_id).await {
        Ok(d) => (StatusCode::ACCEPTED).into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response()
    }
}
