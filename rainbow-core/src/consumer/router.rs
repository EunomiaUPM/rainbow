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

use axum::Router;
use rainbow_auth::ssi::consumer::setup::app::AuthConsumerApplication;
use rainbow_catalog::consumer::setup::application::create_catalog_bypass_consumer_router;
use rainbow_common::config::ApplicationConfig;
// use rainbow_catalog::consumer::setup::application::create_catalog_bypass_consumer_router;
use rainbow_contracts::consumer::setup::application::create_contract_negotiation_consumer_router;
use rainbow_transfer::consumer::setup::application::create_transfer_consumer_router;
// use rainbow_transfer_agent::setup::create_root_http_router;

pub async fn create_core_consumer_router(config: &ApplicationConfig) -> Router {
    let auth_router = AuthConsumerApplication::create_router(&config.ssi_auth()).await;
    let transfer_router = create_transfer_consumer_router(&config.transfer()).await;
    let cn_router = create_contract_negotiation_consumer_router(&config.contracts()).await;
    let catalog_bypass_router = create_catalog_bypass_consumer_router(config.catalog()).await;
    // TODO transfer_agent_router with ApplicationConsumerConfig
    //let transfer_agent_router =
    //    create_root_http_router(&app_config.clone()).await.expect("Failed to create transfer agent router");
    Router::new().merge(transfer_router).merge(cn_router).merge(auth_router).merge(catalog_bypass_router)
    // .merge(transfer_agent_router)
}
