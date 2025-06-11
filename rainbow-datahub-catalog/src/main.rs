use crate::core::datahub_proxy::datahub_proxy::DatahubProxyService;
use crate::core::datahub_proxy::DatahubProxyTrait;
use crate::http::datahub_proxy::datahub_proxy::DataHubProxyRouter;
use crate::http::rainbow_entities::policy_relations_router::{PolicyRelationsRouter, PolicyTemplatesRouter};
use axum::Router;
use rainbow_common::config::provider_config::ApplicationProviderConfig;
use rainbow_datahub_catalog::setup::cmd::CatalogCommands;
use rainbow_db::datahub::repo::sql::DatahubConnectorRepoForSql;
use rainbow_db::datahub::repo::PolicyRelationsRepo;
use sea_orm::{Database, DatabaseConnection, EntityTrait};
use std::sync::Arc;
use tokio;
use tracing::info;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::EnvFilter;

mod core;
mod http;

// #[tokio::main]
// async fn main() {
//     let config = ApplicationProviderConfig::default();
//     let datahub_service = Arc::new(DatahubProxyService::new(config.clone()));
//     let db_connection = Database::connect("postgres://ds_transfer_provider:ds_transfer_provider@127.0.0.1:1300/ds_transfer_provider").await.unwrap();
//
//     let repo = Arc::new(DatahubConnectorRepoForSql::new(db_connection.clone()));
//     let policy_templates_service = Arc::new(DatahubConnectorRepoForSql::new(db_connection.clone()));
//     let policy_relations_service = Arc::new(DatahubConnectorRepoForSql::new(db_connection.clone()));
//
//
//
//     let datahub_router = DataHubProxyRouter::new(datahub_service.clone());
//     let policy_templates_router = PolicyTemplatesRouter::new(policy_templates_service.clone());
//     let policy_relations_router = PolicyRelationsRouter::new(policy_relations_service.clone());
//
//
//
//     let app = Router::new()
//         .merge(datahub_router.router())
//         .merge(policy_templates_router.router())
//         .merge(policy_relations_router.router());
//
//
//     println!("🚀 Servidor corriendo en http://localhost:3000");
//     let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
//     axum::serve(listener, app).await.unwrap();
// }

const INFO: &str = r"
----------
 ____    __    ____  _  _  ____  _____  _    _
(  _ \  /__\  (_  _)( \( )(  _ \(  _  )( \/\/ )
 )   / /(__)\  _)(_  )  (  ) _ < )(_)(  )    (
(_)\_)(__)(__)(____)(_)\_)(____/(_____)(__/\__)

Starting Rainbow Datahub Connection Server 🌈🌈
UPM Dataspace protocol implementation
Show some love on https://github.com/ging/rainbow
----------

";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let filter =
        EnvFilter::builder().with_default_directive(LevelFilter::INFO.into()).parse("debug,sqlx::query=off")?;
    tracing_subscriber::fmt().with_env_filter(filter).init();
    info!("{}", INFO);
    CatalogCommands::init_command_line().await?;
    Ok(())
}