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

use crate::consumer::setup::config::CoreApplicationConsumerConfig;
use axum::Router;
use rainbow_common::config::consumer_config::ApplicationConsumerConfig;
use rainbow_contracts::consumer::setup::application::create_contract_negotiation_consumer_router;
use rainbow_transfer::consumer::setup::application::create_transfer_consumer_router;
use rainbow_auth::ssi_auth::consumer::http::RainbowAuthConsumerRouter;
use rainbow_db::auth_consumer::repo::sql::AuthConsumerRepoForSql;
use rainbow_db::auth_consumer::repo::AuthConsumerRepoFactory;

pub async fn create_core_consumer_router(config: &CoreApplicationConsumerConfig) -> Router {
    // TODO fix this
    let auth_router = RainbowAuthConsumerRouter::new(auth_repo.clone()).router();
    let auth_repo = Arc::new(AuthConsumerRepoForSql::create_repo(db_connection.clone()));


    let app_config: ApplicationConsumerConfig = config.clone().into();
    let transfer_router = create_transfer_consumer_router(&app_config.clone().into()).await;
    let cn_router = create_contract_negotiation_consumer_router(&app_config.clone().into()).await;
    Router::new()
        .merge(transfer_router)
        .merge(cn_router)
}
