use axum::{
    Router,
};
use rainbow_common::config::provider_config::ApplicationProviderConfig;
use std::sync::Arc;
use tokio;
use crate::core::datahub_proxy::datahub_proxy::DatahubProxyService;
use crate::core::datahub_proxy::DatahubProxyTrait;
use crate::http::datahub_proxy::datahub_proxy::DataHubProxyRouter;
use crate::http::rainbow_entities::policy_relations_router::{PolicyTemplatesRouter, PolicyRelationsRouter}; 
use rainbow_db::datahub::repo::sql::DatahubConnectorRepoForSql;
use rainbow_db::datahub::repo::{NewDataHubDatasetModel, DatahubDatasetsRepo, PolicyRelationsRepo};
use sea_orm::{DatabaseConnection, Database, EntityTrait};
mod core;
mod http;

#[tokio::main]
async fn main() {
    let config = ApplicationProviderConfig::default();
    let datahub_service = Arc::new(DatahubProxyService::new(config.clone()));
    let db_connection = Database::connect("postgres://ds_transfer_provider:ds_transfer_provider@127.0.0.1:1300/ds_transfer_provider").await.unwrap();
    
    let repo = Arc::new(DatahubConnectorRepoForSql::new(db_connection.clone()));
    let policy_templates_service = Arc::new(DatahubConnectorRepoForSql::new(db_connection.clone()));
    let policy_relations_service = Arc::new(DatahubConnectorRepoForSql::new(db_connection.clone()));
    
    

    let datahub_router = DataHubProxyRouter::new(datahub_service.clone());
    let policy_templates_router = PolicyTemplatesRouter::new(policy_templates_service.clone());
    let policy_relations_router = PolicyRelationsRouter::new(policy_relations_service.clone());



    let app = Router::new()
        .merge(datahub_router.router())
        .merge(policy_templates_router.router())
        .merge(policy_relations_router.router());    
    

    println!("🚀 Servidor corriendo en http://localhost:3000");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}