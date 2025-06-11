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

use crate::ssi_auth::provider::setup::config::SSIAuthProviderApplicationConfig;
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