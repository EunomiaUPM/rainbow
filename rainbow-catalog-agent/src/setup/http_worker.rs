/*
 *
 *  * Copyright (C) 2025 - Universidad Politécnica de Madrid - UPM
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

use crate::data::factory_sql::CatalogAgentRepoForSql;
use crate::entities::catalogs::catalogs::CatalogEntities;
use crate::entities::data_services::data_services::DataServiceEntities;
use crate::entities::datasets::datasets::DatasetEntities;
use crate::entities::distributions::distributions::DistributionEntities;
use crate::entities::odrl_policies::odrl_policies::OdrlPolicyEntities;
use crate::entities::policy_templates::policy_templates::PolicyTemplateEntities;
use crate::http::catalogs::CatalogEntityRouter;
use crate::http::data_services::DataServiceEntityRouter;
use crate::http::datasets::DatasetEntityRouter;
use crate::http::distributions::DistributionEntityRouter;
use crate::http::odrl_policies::OdrlOfferEntityRouter;
use crate::http::policy_templates::PolicyTemplateEntityRouter;
use crate::protocols::dsp::CatalogDSP;
use crate::protocols::protocol::ProtocolPluginTrait;
use axum::extract::Request;
use axum::response::IntoResponse;
use axum::{serve, Router};
use rainbow_common::config::global_config::ApplicationGlobalConfig;
use rainbow_common::errors::CommonErrors;
use rainbow_common::health::HealthRouter;
use rainbow_common::well_known::WellKnownRoot;
use sea_orm::Database;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;
use tower_http::trace::{DefaultOnResponse, TraceLayer};
use uuid::Uuid;

pub struct CatalogHttpWorker {}
impl CatalogHttpWorker {
    pub async fn spawn(config: &ApplicationGlobalConfig, token: &CancellationToken) -> anyhow::Result<JoinHandle<()>> {
        // well known router
        let well_known_router = WellKnownRoot::get_router()?;
        let health_router = HealthRouter::new().router();
        // module catalog router
        let router = Self::create_root_http_router(&config).await?.merge(well_known_router).merge(health_router);
        let host = if config.is_local { "127.0.0.1" } else { "0.0.0.0" };
        let port = config.catalog_host.clone().expect("no host").port;
        let addr = format!("{}:{}", host, port);

        let listener = TcpListener::bind(&addr).await?;
        tracing::info!("HTTP Catalog Service running on {}", addr);

        let token = token.clone();
        let handle = tokio::spawn(async move {
            let server = serve(listener, router).with_graceful_shutdown(async move {
                token.cancelled().await;
                tracing::info!("HTTP Service received shutdown signal, draining connections...");
            });
            match server.await {
                Ok(_) => tracing::info!("HTTP Service stopped successfully"),
                Err(e) => tracing::error!("HTTP Service crashed: {}", e),
            }
        });

        Ok(handle)
    }
    pub async fn create_root_http_router(config: &ApplicationGlobalConfig) -> anyhow::Result<Router> {
        let router = create_root_http_router(config).await?.fallback(Self::handler_404).layer(
            TraceLayer::new_for_http()
                .make_span_with(|_req: &Request<_>| tracing::info_span!("request", id = %Uuid::new_v4()))
                .on_request(|request: &Request<_>, _span: &tracing::Span| {
                    tracing::info!("{} {}", request.method(), request.uri());
                })
                .on_response(DefaultOnResponse::new().level(tracing::Level::TRACE)),
        );
        Ok(router)
    }
    async fn handler_404(uri: axum::http::Uri) -> impl IntoResponse {
        let err = CommonErrors::missing_resource_new(&uri.to_string(), "Route not found or Method not allowed");
        tracing::info!("404 Not Found: {}", uri);
        err.into_response()
    }
}

pub async fn create_root_http_router(config: &ApplicationGlobalConfig) -> anyhow::Result<Router> {
    // ROOT Dependency Injection
    let config = Arc::new(config.clone());
    let db_connection = Database::connect(config.database_config.as_db_url()).await.expect("Database can't connect");

    let catalog_agent_repo = Arc::new(CatalogAgentRepoForSql::create_repo(db_connection.clone()));

    // entities
    let catalog_controller_service = Arc::new(CatalogEntities::new(catalog_agent_repo.clone()));
    let catalog_router = CatalogEntityRouter::new(catalog_controller_service.clone(), config.clone());
    let data_services_controller_service = Arc::new(DataServiceEntities::new(catalog_agent_repo.clone()));
    let data_services_router = DataServiceEntityRouter::new(data_services_controller_service.clone(), config.clone());
    let datasets_controller_service = Arc::new(DatasetEntities::new(catalog_agent_repo.clone()));
    let datasets_router = DatasetEntityRouter::new(datasets_controller_service.clone(), config.clone());
    let distributions_controller_service = Arc::new(DistributionEntities::new(catalog_agent_repo.clone()));
    let distributions_router = DistributionEntityRouter::new(distributions_controller_service.clone(), config.clone());
    let odrl_offer_controller_service = Arc::new(OdrlPolicyEntities::new(catalog_agent_repo.clone()));
    let odrl_offer_router = OdrlOfferEntityRouter::new(odrl_offer_controller_service.clone(), config.clone());
    let policy_templates_controller_service = Arc::new(PolicyTemplateEntities::new(catalog_agent_repo.clone()));
    let policy_templates_router =
        PolicyTemplateEntityRouter::new(policy_templates_controller_service.clone(), config.clone());

    // dsp
    let dsp_router = CatalogDSP::new(
        catalog_controller_service.clone(),
        data_services_controller_service.clone(),
        datasets_controller_service.clone(),
        odrl_offer_controller_service.clone(),
        distributions_controller_service.clone(),
        config.clone(),
    )
    .build_router()
    .await?;

    let router_str = format!("/api/{}/catalog-agent", config.api_version);
    let router = Router::new()
        .nest(
            format!("{}/catalogs", router_str.as_str()).as_str(),
            catalog_router.router(),
        )
        .nest(
            format!("{}/data-services", router_str.as_str()).as_str(),
            data_services_router.router(),
        )
        .nest(
            format!("{}/datasets", router_str.as_str()).as_str(),
            datasets_router.router(),
        )
        .nest(
            format!("{}/distributions", router_str.as_str()).as_str(),
            distributions_router.router(),
        )
        .nest(
            format!("{}/odrl-policies", router_str.as_str()).as_str(),
            odrl_offer_router.router(),
        )
        .nest(
            format!("{}/policy-templates", router_str.as_str()).as_str(),
            policy_templates_router.router(),
        )
        .nest(
            format!("{}/dsp/current/catalog", router_str.as_str()).as_str(),
            dsp_router,
        );

    Ok(router)
}
