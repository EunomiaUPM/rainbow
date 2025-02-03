/*
 *
 *  * Copyright (C) 2024 - Universidad Politécnica de Madrid - UPM
 *  *
 *  * This program is free software: you can redistribute it and/or modify
 *  * it under the terms of the GNU General Public License as published by
 *  * the Free Software Foundation, either version 3 of the License, or
 *  * (at your option) any later version.
 *  *
 *  * This program is distributed in the hope that it will be useful,
 *  * but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  * GNU General Public License for more details.
 *  *
 *  * You should have received a copy of the GNU General Public License
 *  * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 *
 */

use crate::core::idsa_api::catalog_request;
use crate::core::rainbow_catalog_api::{
    catalog_request_by_id, delete_catalog, delete_dataservice, delete_dataset, delete_distribution,
    get_dataservice_by_id, get_dataset_by_id, get_distribution_by_id,
    get_distributions_by_dataset_id, post_catalog, post_dataservice, post_dataset,
    post_distribution, put_catalog, put_dataservice, put_dataset, put_distribution,
};
use crate::core::rainbow_catalog_err::CatalogError;
use crate::core::rainbow_catalog_types::{
    EditDataServiceRequest, EditDistributionRequest, NewCatalogRequest, NewDataServiceRequest,
    NewDatasetRequest, NewDistributionRequest,
};
use axum::extract::Path;
use axum::http::Uri;
use axum::response::IntoResponse;
use axum::routing::{delete, get, post, put};
use axum::{Json, Router};
use rainbow_common::utils::get_urn_from_string;
use reqwest::StatusCode;
use tower_http::trace::TraceLayer;
use tracing::info;

pub async fn catalog_api_router() -> anyhow::Result<Router> {
    let router = Router::new()
        // CATALOGS
        .route("/api/v1/catalogs", get(handle_get_catalogs))
        .route("/api/v1/catalogs/:id", get(handle_get_catalogs_by_id))
        .route("/api/v1/catalogs", post(handle_post_catalog))
        .route("/api/v1/catalogs/:id", put(handle_put_catalog))
        .route("/api/v1/catalogs/:id", delete(handle_delete_catalog))
        // DATASETS
        .route("/api/v1/datasets/:id", get(handle_get_dataset_by_id))
        .route("/api/v1/catalogs/:id/datasets", post(handle_post_dataset))
        .route(
            "/api/v1/catalogs/:id/datasets/:did",
            put(handle_put_dataset),
        )
        .route(
            "/api/v1/catalogs/:id/datasets/:did",
            delete(handle_delete_dataset),
        )
        // DISTRIBUTIONS
        .route(
            "/api/v1/distributions/:id",
            get(handle_get_distributions_by_id),
        )
        .route(
            "/api/v1/datasets/:id/distributions",
            get(handle_get_distributions_by_dataset_id),
        )
        .route(
            "/api/v1/catalogs/:id/datasets/:did/distributions",
            post(handle_post_distribution),
        )
        .route(
            "/api/v1/catalogs/:id/datasets/:did/distributions/:ddid",
            put(handle_put_distribution),
        )
        .route(
            "/api/v1/catalogs/:id/datasets/:did/distributions/:ddid",
            delete(handle_delete_distribution),
        )
        // DATASERVICES
        .route(
            "/api/v1/data-services/:id",
            get(handle_get_dataservice_by_id),
        )
        .route(
            "/api/v1/catalogs/:id/data-services",
            post(handle_post_dataservice),
        )
        .route(
            "/api/v1/catalogs/:id/data-services/:did",
            put(handle_put_dataservice),
        )
        .route(
            "/api/v1/catalogs/:id/data-services/:did",
            delete(handle_delete_dataservice),
        );
    Ok(router)
}

async fn handle_get_catalogs() -> impl IntoResponse {
    info!("GET /api/v1/catalogs");

    match catalog_request().await {
        Ok(c) => (StatusCode::OK, Json(c)).into_response(),
        Err(err) => match err.downcast::<CatalogError>() {
            Ok(e) => e.into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        },
    }
}

async fn handle_get_catalogs_by_id(Path(id): Path<String>) -> impl IntoResponse {
    info!("GET /api/v1/catalogs/{}", id);
    let catalog_id = match get_urn_from_string(&id) {
        Ok(catalog_id) => catalog_id,
        Err(err) => return (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
    };
    match catalog_request_by_id(catalog_id).await {
        Ok(c) => (StatusCode::OK, Json(c)).into_response(),
        Err(err) => match err.downcast::<CatalogError>() {
            Ok(e) => e.into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        },
    }
}

async fn handle_post_catalog(Json(input): Json<NewCatalogRequest>) -> impl IntoResponse {
    info!("POST /api/v1/catalogs");
    match post_catalog(input).await {
        Ok(c) => (StatusCode::CREATED, Json(c)).into_response(),
        Err(err) => match err.downcast::<CatalogError>() {
            Ok(e) => e.into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        },
    }
}

async fn handle_put_catalog(
    Path(id): Path<String>,
    Json(input): Json<NewCatalogRequest>,
) -> impl IntoResponse {
    info!("PUT /api/v1/catalogs/{}", id);
    let catalog_id = match get_urn_from_string(&id) {
        Ok(catalog_id) => catalog_id,
        Err(err) => return (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
    };
    match put_catalog(catalog_id, input).await {
        Ok(c) => (StatusCode::ACCEPTED, Json(c)).into_response(),
        Err(err) => match err.downcast::<CatalogError>() {
            Ok(e) => e.into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        },
    }
}

async fn handle_delete_catalog(Path(id): Path<String>) -> impl IntoResponse {
    info!("DELETE /api/v1/catalogs/{}", id);
    let catalog_id = match get_urn_from_string(&id) {
        Ok(catalog_id) => catalog_id,
        Err(err) => return (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
    };
    match delete_catalog(catalog_id).await {
        Ok(_) => (StatusCode::ACCEPTED).into_response(),
        Err(err) => match err.downcast::<CatalogError>() {
            Ok(e) => e.into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        },
    }
}

async fn handle_get_dataset_by_id(Path(id): Path<String>) -> impl IntoResponse {
    info!("GET /api/v1/datasets/:id");
    let dataset_id = match get_urn_from_string(&id) {
        Ok(id) => id,
        Err(err) => return (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
    };
    match get_dataset_by_id(dataset_id).await {
        Ok(d) => (StatusCode::OK, Json(d)).into_response(),
        Err(err) => match err.downcast::<CatalogError>() {
            Ok(e) => e.into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        },
    }
}

async fn handle_post_dataset(
    Path(id): Path<String>,
    Json(input): Json<NewDatasetRequest>,
) -> impl IntoResponse {
    info!("POST /api/v1/catalogs/{}/datasets", id);
    let dataset_id = match get_urn_from_string(&id) {
        Ok(id) => id,
        Err(err) => return (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
    };
    match post_dataset(dataset_id, input).await {
        Ok(d) => (StatusCode::CREATED, Json(d)).into_response(),
        Err(err) => match err.downcast::<CatalogError>() {
            Ok(e) => e.into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        },
    }
}

async fn handle_put_dataset(
    Path((c_id, d_id)): Path<(String, String)>,
    Json(input): Json<NewDatasetRequest>,
) -> impl IntoResponse {
    info!("PUT /api/v1/catalogs/{}/datasets/{}", c_id, d_id);
    let catalog_id = match get_urn_from_string(&c_id) {
        Ok(id) => id,
        Err(err) => return (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
    };
    let dataset_id = match get_urn_from_string(&d_id) {
        Ok(id) => id,
        Err(err) => return (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
    };
    match put_dataset(catalog_id, dataset_id, input).await {
        Ok(d) => (StatusCode::ACCEPTED, Json(d)).into_response(),
        Err(err) => match err.downcast::<CatalogError>() {
            Ok(e) => e.into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        },
    }
}

async fn handle_delete_dataset(Path((c_id, d_id)): Path<(String, String)>) -> impl IntoResponse {
    info!("DELETE /api/v1/catalogs/{}/datasets/{}", c_id, d_id);
    let catalog_id = match get_urn_from_string(&c_id) {
        Ok(id) => id,
        Err(err) => return (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
    };
    let dataset_id = match get_urn_from_string(&d_id) {
        Ok(id) => id,
        Err(err) => return (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
    };
    match delete_dataset(catalog_id, dataset_id).await {
        Ok(d) => (StatusCode::ACCEPTED).into_response(),
        Err(err) => match err.downcast::<CatalogError>() {
            Ok(e) => e.into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        },
    }
}

async fn handle_get_distributions_by_id(Path(id): Path<String>) -> impl IntoResponse {
    info!("GET /api/v1/distributions/:id");
    let distribution_id = match get_urn_from_string(&id) {
        Ok(id) => id,
        Err(err) => return (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
    };
    match get_distribution_by_id(distribution_id).await {
        Ok(d) => (StatusCode::OK, Json(d)).into_response(),
        Err(err) => match err.downcast::<CatalogError>() {
            Ok(e) => e.into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        },
    }
}

async fn handle_get_distributions_by_dataset_id(Path(id): Path<String>) -> impl IntoResponse {
    info!("GET /api/v1/datasets/{}/distributions", id);
    let dataset_id = match get_urn_from_string(&id) {
        Ok(id) => id,
        Err(err) => return (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
    };
    match get_distributions_by_dataset_id(dataset_id).await {
        Ok(d) => (StatusCode::OK, Json(d)).into_response(),
        Err(err) => match err.downcast::<CatalogError>() {
            Ok(e) => e.into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        },
    }
}

async fn handle_post_distribution(
    Path((id, did)): Path<(String, String)>,
    Json(input): Json<NewDistributionRequest>,
) -> impl IntoResponse {
    info!("POST /api/v1/catalogs/{}/distributions", id);
    let catalog_id = match get_urn_from_string(&id) {
        Ok(id) => id,
        Err(err) => return (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
    };
    let distribution_id = match get_urn_from_string(&did) {
        Ok(id) => id,
        Err(err) => return (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
    };
    match post_distribution(catalog_id, distribution_id, input).await {
        Ok(d) => (StatusCode::CREATED, Json(d)).into_response(),
        Err(err) => match err.downcast::<CatalogError>() {
            Ok(e) => e.into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        },
    }
}

async fn handle_put_distribution(
    Path((c_id, d_id, ds_id)): Path<(String, String, String)>,
    Json(input): Json<EditDistributionRequest>,
) -> impl IntoResponse {
    info!(
        "PUT /api/v1/catalogs/{}/datasets/{}/distributions/{}",
        c_id, d_id, ds_id
    );
    let catalog_id = match get_urn_from_string(&c_id) {
        Ok(id) => id,
        Err(err) => return (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
    };
    let dataset_id = match get_urn_from_string(&d_id) {
        Ok(id) => id,
        Err(err) => return (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
    };
    let distribution_id = match get_urn_from_string(&ds_id) {
        Ok(id) => id,
        Err(err) => return (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
    };
    match put_distribution(catalog_id, dataset_id, distribution_id, input).await {
        Ok(d) => (StatusCode::ACCEPTED, Json(d)).into_response(),
        Err(err) => match err.downcast::<CatalogError>() {
            Ok(e) => e.into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        },
    }
}

async fn handle_delete_distribution(
    Path((c_id, d_id, ds_id)): Path<(String, String, String)>,
) -> impl IntoResponse {
    info!(
        "DELETE /api/v1/catalogs/{}/datasets/{}/distributions/{}",
        c_id, d_id, ds_id
    );
    let catalog_id = match get_urn_from_string(&c_id) {
        Ok(id) => id,
        Err(err) => return (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
    };
    let dataset_id = match get_urn_from_string(&d_id) {
        Ok(id) => id,
        Err(err) => return (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
    };
    let distribution_id = match get_urn_from_string(&ds_id) {
        Ok(id) => id,
        Err(err) => return (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
    };
    match delete_distribution(catalog_id, dataset_id, distribution_id).await {
        Ok(d) => (StatusCode::ACCEPTED).into_response(),
        Err(err) => match err.downcast::<CatalogError>() {
            Ok(e) => e.into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        },
    }
}

async fn handle_get_dataservice_by_id(Path(id): Path<String>) -> impl IntoResponse {
    info!("GET /api/v1/data-services/:id");
    let dataservice_id = match get_urn_from_string(&id) {
        Ok(id) => id,
        Err(err) => return (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
    };
    match get_dataservice_by_id(dataservice_id).await {
        Ok(d) => (StatusCode::OK, Json(d)).into_response(),
        Err(err) => match err.downcast::<CatalogError>() {
            Ok(e) => e.into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        },
    }
}

async fn handle_post_dataservice(
    Path(id): Path<String>,
    Json(input): Json<NewDataServiceRequest>,
) -> impl IntoResponse {
    info!("POST /api/v1/catalogs/{}/data-services", id);
    let dataservice_id = match get_urn_from_string(&id) {
        Ok(id) => id,
        Err(err) => return (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
    };
    match post_dataservice(dataservice_id, input).await {
        Ok(d) => (StatusCode::CREATED, Json(d)).into_response(),
        Err(err) => match err.downcast::<CatalogError>() {
            Ok(e) => e.into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        },
    }
}

async fn handle_put_dataservice(
    Path((c_id, ds_id)): Path<(String, String)>,
    Json(input): Json<EditDataServiceRequest>,
) -> impl IntoResponse {
    info!("PUT /api/v1/catalogs/{}/data-services/{}", c_id, ds_id);
    let catalog_id = match get_urn_from_string(&c_id) {
        Ok(id) => id,
        Err(err) => return (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
    };
    let dataservice_id = match get_urn_from_string(&ds_id) {
        Ok(id) => id,
        Err(err) => return (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
    };
    match put_dataservice(catalog_id, dataservice_id, input).await {
        Ok(d) => (StatusCode::ACCEPTED, Json(d)).into_response(),
        Err(err) => match err.downcast::<CatalogError>() {
            Ok(e) => e.into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        },
    }
}

async fn handle_delete_dataservice(
    Path((c_id, ds_id)): Path<(String, String)>,
) -> impl IntoResponse {
    info!("DELETE /api/v1/catalogs/{}/data-services/{}", c_id, ds_id);
    let catalog_id = match get_urn_from_string(&c_id) {
        Ok(id) => id,
        Err(err) => return (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
    };
    let dataservice_id = match get_urn_from_string(&ds_id) {
        Ok(id) => id,
        Err(err) => return (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
    };
    match delete_dataservice(catalog_id, dataservice_id).await {
        Ok(d) => (StatusCode::ACCEPTED).into_response(),
        Err(err) => match err.downcast::<CatalogError>() {
            Ok(e) => e.into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        },
    }
}
