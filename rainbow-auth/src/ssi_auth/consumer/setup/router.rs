
use crate::ssi_auth::consumer::setup::config::SSIAuthConsumerApplicationConfig;
use crate::ssi_auth::consumer::core::Manager;
use crate::ssi_auth::consumer::http::http::RainbowAuthConsumerRouter;
use axum::Router;
use rainbow_common::config::consumer_config::ApplicationConsumerConfigTrait;
use rainbow_db::auth_consumer::repo::sql::AuthConsumerRepoForSql;
use rainbow_db::auth_consumer::repo::AuthConsumerRepoFactory;
use sea_orm::Database;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn create_ssi_consumer_router(config: SSIAuthConsumerApplicationConfig) -> Router {
    let db_connection = Database::connect(config.get_full_db_url()).await.expect("Database can't connect");
    let auth_repo = Arc::new(AuthConsumerRepoForSql::create_repo(db_connection));
    let manager = Arc::new(Mutex::new(Manager::new(auth_repo.clone(), config.clone())));
    let auth_router = RainbowAuthConsumerRouter::new(manager.clone()).router();
    auth_router
}
