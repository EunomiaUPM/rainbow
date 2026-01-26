use crate::data::factory_sql::ConnectorRepoForSql;
use crate::data::factory_trait::ConnectorRepoTrait;
use crate::entities::connector_instance::connector_instance::ConnectorInstanceEntitiesService;
use crate::entities::connector_template::connector_template::ConnectorTemplateEntitiesService;
use crate::http::connector_instance::ConnectorInstanceRouter;
use crate::http::connector_template::ConnectorTemplateRouter;
use axum::Router;
use rainbow_common::config::services::CatalogConfig;
use rainbow_common::config::traits::DatabaseConfigTrait;
use sea_orm::Database;
use std::sync::Arc;

pub struct ConnectorSetup {}
impl ConnectorSetup {
    pub fn new() -> Self {
        ConnectorSetup {}
    }
    pub async fn get_connector_repo(&self, config: &CatalogConfig) -> Arc<dyn ConnectorRepoTrait> {
        let db_url = config.get_full_db_url();
        let db_connection = Database::connect(db_url).await.expect("Database can't connect");
        let connector_repo = Arc::new(ConnectorRepoForSql::create_repo(db_connection));
        connector_repo
    }
    pub async fn build_control_router(&self, config: &CatalogConfig) -> Router {
        let connector_repo = self.get_connector_repo(config).await;
        let config = Arc::new(config.clone());
        let connector_template_service = Arc::new(ConnectorTemplateEntitiesService::new(
            connector_repo.clone(),
        ));
        let connector_template_router =
            ConnectorTemplateRouter::new(connector_template_service.clone(), config.clone()).router();
        let connector_instance_service = Arc::new(ConnectorInstanceEntitiesService::new(
            connector_repo.clone(),
        ));
        let connector_instance_router = ConnectorInstanceRouter::new(connector_instance_service.clone()).router();
        Router::new().nest("/templates", connector_template_router).nest("/instances", connector_instance_router)
    }
}
