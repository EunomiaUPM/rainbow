use crate::setup::provider::SSIAuthProviderApplicationConfig;
use crate::ssi_auth::provider::core::manager::manager::Manager;
use crate::ssi_auth::provider::http::http::RainbowAuthProviderRouter;
use axum::Router;
use rainbow_common::config::provider_config::ApplicationProviderConfigTrait;
use rainbow_db::auth_provider::repo::sql::AuthProviderRepoForSql;
use rainbow_db::auth_provider::repo::AuthProviderRepoFactory;
use sea_orm::Database;
use std::sync::Arc;

pub async fn create_ssi_provider_router(config: SSIAuthProviderApplicationConfig) -> Router {
    let db_connection = Database::connect(config.get_full_db_url()).await.expect("Database can't connect");
    let auth_repo = Arc::new(AuthProviderRepoForSql::create_repo(db_connection));
    let manager = Arc::new(Manager::new(auth_repo.clone(), config.clone()));
    let auth_router = RainbowAuthProviderRouter::new(manager.clone()).router();
    auth_router
}