use crate::core::hl_api::{catalog_request_by_id, delete_catalog, delete_dataservice, delete_dataset, delete_distribution, post_catalog, post_dataservice, post_dataset, post_distribution, put_catalog, put_dataservice, put_dataset, put_distribution, EditDataServiceRequest, EditDistributionRequest, NewCatalogRequest, NewDataServiceRequest, NewDatasetRequest, NewDistributionRequest};
use crate::core::ll_api::catalog_request;
use axum::extract::Path;
use axum::response::IntoResponse;
use axum::routing::{delete, get, post, put};
use axum::{Json, Router};
use reqwest::StatusCode;
use tower_http::trace::TraceLayer;
use tracing::info;
use uuid::Uuid;

pub async fn catalog_api_router() -> anyhow::Result<Router> {
    let router = Router::new()
        .route("/api/v1/catalogs", get(handle_get_catalogs))
        .route("/api/v1/catalogs/:id", get(handle_get_catalogs_by_id))
        .route("/api/v1/catalogs", post(handle_post_catalog))
        .route("/api/v1/catalogs/:id", put(handle_put_catalog))
        .route("/api/v1/catalogs/:id", delete(handle_delete_catalog))
        // TODO getters for other instances
        .route("/api/v1/catalogs/:id/datasets", post(handle_post_dataset))
        .route("/api/v1/catalogs/:id/datasets/:did", put(handle_put_dataset))
        .route("/api/v1/catalogs/:id/datasets/:did", delete(handle_delete_dataset))
        //
        .route("/api/v1/catalogs/:id/datasets/:did/distributions", post(handle_post_distribution))
        .route("/api/v1/catalogs/:id/datasets/:did/distributions/:ddid", put(handle_put_distribution))
        .route("/api/v1/catalogs/:id/datasets/:did/distributions/:ddid", delete(handle_delete_distribution))
        //
        .route("/api/v1/catalogs/:id/data-services", post(handle_post_dataservice))
        .route("/api/v1/catalogs/:id/data-services/:did", put(handle_put_dataservice))
        .route("/api/v1/catalogs/:id/data-services/:did", delete(handle_delete_dataservice));
        //
    Ok(router)
}

async fn handle_get_catalogs() -> impl IntoResponse {
    info!("GET /api/v1/catalogs");

    match catalog_request().await {
        Ok(c) => (StatusCode::OK, Json(c)).into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
    }
}

async fn handle_get_catalogs_by_id(
    Path(id): Path<Uuid>
) -> impl IntoResponse {
    info!("GET /api/v1/catalogs/{}", id.to_string());
    match catalog_request_by_id(id).await {
        Ok(c) => (StatusCode::OK, Json(c)).into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
    }
}

async fn handle_post_catalog(
    Json(input): Json<NewCatalogRequest>
) -> impl IntoResponse {
    info!("POST /api/v1/catalogs");
    match post_catalog(input).await {
        Ok(c) => (StatusCode::CREATED, Json(c)).into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
    }
}

async fn handle_put_catalog(
    Path(id): Path<Uuid>,
    Json(input): Json<NewCatalogRequest>,
) -> impl IntoResponse {
    info!("PUT /api/v1/catalogs/{}", id.to_string());
    match put_catalog(id, input).await {
        Ok(c) => (StatusCode::ACCEPTED, Json(c)).into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
    }
}

async fn handle_delete_catalog(
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    info!("DELETE /api/v1/catalogs/{}", id.to_string());
    match delete_catalog(id).await {
        Ok(_) => (StatusCode::ACCEPTED).into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
    }
}

async fn handle_post_dataset(
    Path(id): Path<Uuid>,
    Json(input): Json<NewDatasetRequest>,
) -> impl IntoResponse {
    info!("POST /api/v1/catalogs/{}/datasets", id.to_string());
    match post_dataset(id, input).await {
        Ok(d) => (StatusCode::CREATED, Json(d)).into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
    }
}

async fn handle_put_dataset(
    Path(id): Path<(Uuid, Uuid)>,
    Json(input): Json<NewDatasetRequest>,
) -> impl IntoResponse {
    info!("PUT /api/v1/catalogs/{}/datasets/{}", id.0.to_string(), id.1.to_string());
    match put_dataset(id.0, id.1, input).await {
        Ok(d) => (StatusCode::ACCEPTED, Json(d)).into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
    }
}

async fn handle_delete_dataset(
    Path(id): Path<(Uuid, Uuid)>,
) -> impl IntoResponse {
    info!("DELETE /api/v1/catalogs/{}/datasets/{}", id.0.to_string(), id.1.to_string());
    match delete_dataset(id.0, id.1).await {
        Ok(d) => (StatusCode::ACCEPTED).into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
    }
}


async fn handle_post_distribution(
    Path((id, did)): Path<(Uuid, Uuid)>,
    Json(input): Json<NewDistributionRequest>,
) -> impl IntoResponse {
    info!("POST /api/v1/catalogs/{}/datasets", id.to_string());
    match post_distribution(id, did, input).await {
        Ok(d) => (StatusCode::CREATED, Json(d)).into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
    }
}

async fn handle_put_distribution(
    Path(id): Path<(Uuid, Uuid, Uuid)>,
    Json(input): Json<EditDistributionRequest>,
) -> impl IntoResponse {
    info!("PUT /api/v1/catalogs/{}/datasets/{}", id.0.to_string(), id.1.to_string());
    match put_distribution(id.0, id.1, id.2, input).await {
        Ok(d) => (StatusCode::ACCEPTED, Json(d)).into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
    }
}

async fn handle_delete_distribution(
    Path(id): Path<(Uuid, Uuid, Uuid)>,
) -> impl IntoResponse {
    info!("DELETE /api/v1/catalogs/{}/datasets/{}", id.0.to_string(), id.1.to_string());
    match delete_distribution(id.0, id.1, id.2).await {
        Ok(d) => (StatusCode::ACCEPTED).into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
    }
}


async fn handle_post_dataservice(
    Path(id): Path<Uuid>,
    Json(input): Json<NewDataServiceRequest>,
) -> impl IntoResponse {
    info!("POST /api/v1/catalogs/{}/data-services", id.to_string());
    match post_dataservice(id, input).await {
        Ok(d) => (StatusCode::CREATED, Json(d)).into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
    }
}

async fn handle_put_dataservice(
    Path(id): Path<(Uuid, Uuid)>,
    Json(input): Json<EditDataServiceRequest>,
) -> impl IntoResponse {
    info!("PUT /api/v1/catalogs/{}/data-services/{}", id.0.to_string(), id.1.to_string());
    match put_dataservice(id.0, id.1, input).await {
        Ok(d) => (StatusCode::ACCEPTED, Json(d)).into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
    }
}

async fn handle_delete_dataservice(
    Path(id): Path<(Uuid, Uuid)>,
) -> impl IntoResponse {
    info!("DELETE /api/v1/catalogs/{}/data-services/{}", id.0.to_string(), id.1.to_string());
    match delete_dataservice(id.0, id.1).await {
        Ok(d) => (StatusCode::ACCEPTED).into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
    }
}
