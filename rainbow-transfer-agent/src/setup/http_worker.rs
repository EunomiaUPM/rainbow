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

use crate::data::factory_sql::TransferAgentRepoForSql;
use crate::entities::transfer_messages::transfer_messages::TransferAgentMessagesService;
use crate::entities::transfer_process::transfer_process::TransferAgentProcessesService;
use crate::http::transfer_messages::TransferAgentMessagesRouter;
use crate::http::transfer_process::TransferAgentProcessesRouter;
use crate::protocols::dsp::TransferDSP;
use crate::protocols::protocol::ProtocolPluginTrait;
use axum::extract::Request;
use axum::response::IntoResponse;
use axum::{serve, Router};
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
use rainbow_common::config::global_config::ApplicationGlobalConfig;

pub struct TransferHttpWorker {}
impl TransferHttpWorker {
    pub async fn spawn(
        config: &ApplicationGlobalConfig,
        token: &CancellationToken,
    ) -> anyhow::Result<JoinHandle<()>> {
        // well known router
        let well_known_router = WellKnownRoot::get_router()?;
        let health_router = HealthRouter::new().router();
        // module transfer router
        let router = Self::create_root_http_router(&config).await?.merge(well_known_router).merge(health_router);
        let host = if config.is_local { "127.0.0.1" } else { "0.0.0.0" };
        let port = config.transfer_process_host.clone().expect("no host").port;
        let addr = format!("{}:{}", host, port);

        let listener = TcpListener::bind(&addr).await?;
        tracing::info!("HTTP Transfer Service running on {}", addr);

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
    let transfer_repo = Arc::new(TransferAgentRepoForSql::create_repo(db_connection.clone()));

    // entities
    let messages_controller_service = Arc::new(TransferAgentMessagesService::new(transfer_repo.clone()));
    let messages_router = TransferAgentMessagesRouter::new(messages_controller_service.clone(), config.clone());
    let entities_controller_service = Arc::new(TransferAgentProcessesService::new(transfer_repo.clone()));
    let entities_router = TransferAgentProcessesRouter::new(entities_controller_service.clone(), config.clone());

    // dsp
    let dsp_router = TransferDSP::new(
        messages_controller_service.clone(),
        entities_controller_service.clone(),
        config.clone()
    )
    .build_router()
    .await?;

    let router_str = format!("/api/{}/transfer-agent", config.api_version);
    let router = Router::new()
        .nest(
            format!("{}/transfer-messages", router_str.as_str()).as_str(),
            messages_router.router(),
        )
        .nest(
            format!("{}/transfer-processes", router_str.as_str()).as_str(),
            entities_router.router(),
        )
        .nest(
            format!("{}/dsp/current/transfers", router_str.as_str()).as_str(),
            dsp_router,
        );
    Ok(router)
}
