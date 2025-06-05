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

use crate::consumer::http::http::RainbowMateConsumerRouter;
use crate::consumer::setup::config::MateConsumerApplicationConfig;
use axum::Router;
use rainbow_common::config::consumer_config::ApplicationConsumerConfigTrait;
use rainbow_db::mates::repo::sql::MateRepoForSql;
use rainbow_db::mates::repo::MateRepoFactory;
use sea_orm::Database;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn create_mate_consumer_router(config: MateConsumerApplicationConfig) -> Router {
    let db_connection = Database::connect(config.get_full_db_url()).await.expect("Database can't connect");
    let mate_repo = Arc::new(MateRepoForSql::create_repo(db_connection));
    let mate_router = RainbowMateConsumerRouter::new(mate_repo).router();
    mate_router
}
