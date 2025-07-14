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


use crate::core::datahub_proxy::datahub_proxy_types::DatasetsQueryOptions;
use crate::core::datahub_proxy::datahub_proxy_types::{DomainsQueryOptions, TagsQueryOptions, Catalogs};
use crate::core::datahub_proxy::DatahubProxyTrait;
use axum::extract::{Path, Query, State};
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use reqwest::StatusCode;
use std::sync::Arc;
use tracing::info;
use crate::core::datahub_proxy::datahub_proxy_types::DatahubDataset;
use serde::Serialize;



pub struct DataHubProxyRouter<T>
where
    T: DatahubProxyTrait + Send + Sync + 'static,
{
    datahub_service: Arc<T>,
}


impl<T> DataHubProxyRouter<T>
where
    T: DatahubProxyTrait + Send + Sync + 'static,
{
    pub fn new(datahub_service: Arc<T>) -> Self {
        Self {
            datahub_service
        }
    }
    pub fn router(self) -> Router {
        Router::new()
            .route("/api/v1/datahub/domains", get(Self::handle_get_datahub_domains))
            .route("/api/v1/datahub/tags", get(Self::handle_get_datahub_tags))
            // .route("/api/v1/datahub/domains/:domain_id", get(Self::handle_get_datahub_domain_by_id))
            .route("/api/v1/datahub/domains/:domain_id/datasets", get(Self::handle_get_datasets_by_domain_id))
            .route("/api/v1/datahub/datasets/:dataset_id", get(Self::handle_get_datasets_by_id))
            .route("/api/v1/datahub/domains_with_datasets", get(Self::handle_get_domains_with_datasets))
            .with_state(self.datahub_service)
    }
    async fn handle_get_datahub_domains(
        State(datahub_service): State<Arc<T>>,
        _query: Query<DomainsQueryOptions>,
    ) -> impl IntoResponse {
        info!("GET /api/v1/datahub/domains");
        match datahub_service.get_datahub_domains().await {
            Ok(domains) => (StatusCode::OK, Json(domains)).into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        }
    }

    async fn handle_get_datahub_tags(
        State(datahub_service): State<Arc<T>>,
        Query(options): Query<TagsQueryOptions>,
    ) -> impl IntoResponse {
        info!("GET /api/v1/datahub/tags");
        match datahub_service.get_datahub_tags().await {
            Ok(tags) => (StatusCode::OK, Json(tags)).into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        }
    }

    async fn handle_get_datasets_by_domain_id(
        State(datahub_service): State<Arc<T>>,
        Path(domain_id): Path<String>,
        _query: Query<DatasetsQueryOptions>,
    ) -> impl IntoResponse {
        info!("GET /api/v1/datahub/domains/{}/datasets", domain_id);
        match datahub_service.get_datahub_datasets_by_domain_id(domain_id).await {
            Ok(datasets) => (StatusCode::OK, Json(datasets)).into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        }
    }
    async fn handle_get_datasets_by_id(
        State(datahub_service): State<Arc<T>>,
        Path(dataset_id): Path<String>,
    ) -> impl IntoResponse {
        info!("GET /api/v1/datahub/datasets/{}", dataset_id);
        match datahub_service.get_datahub_dataset_by_id(dataset_id).await {
            Ok(dataset) => (StatusCode::OK, Json(dataset)).into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        }
    }

    async fn handle_get_domains_with_datasets(
        State(datahub_service): State<Arc<T>>,
    ) -> impl IntoResponse {
        use futures::future::join_all;
        info!("GET /api/v1/datahub/domains_with_datasets");
        match datahub_service.get_datahub_domains().await {
            Ok(domains) => {
                let mut results = Vec::new();
                for domain in &domains {
                    let datasets = match datahub_service.get_datahub_datasets_by_domain_id(domain.urn.clone()).await {
                        Ok(ds) => ds,
                        Err(_) => Vec::new(),
                    };
                    let dataset_futures = datasets.into_iter().map(|d| {
                        let datahub_service = datahub_service.clone();
                        let urn = d.urn.clone();
                        async move {
                            match datahub_service.get_datahub_dataset_by_id(urn).await {
                                Ok(detail) => Some(detail),
                                Err(_) => None,
                            }
                        }
                    });
                    let detailed_datasets: Vec<DatahubDataset> = join_all(dataset_futures).await.into_iter().filter_map(|d| d).collect();

                    let catalog = Catalogs {
                        urn: domain.urn.clone(),
                        name: domain.properties.name.clone(),
                        description: domain.properties.description.clone(),
                        datasets: detailed_datasets,
                    };
                    results.push(catalog);
                }
                (StatusCode::OK, Json(results)).into_response()
            },
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        }
    }
}