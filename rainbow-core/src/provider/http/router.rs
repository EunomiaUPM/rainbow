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
use crate::provider::setup::config::CoreApplicationProviderConfig;
use axum::Router;
use rainbow_auth::ssi_auth::provider::setup::router::create_ssi_provider_router;
use rainbow_catalog::setup::application::create_catalog_router;
use rainbow_common::config::provider_config::ApplicationProviderConfig;
use rainbow_contracts::provider::setup::application::create_contract_negotiation_provider_router;
use rainbow_mates::provider::setup::application::create_mate_consumer_router;
use rainbow_transfer::provider::setup::application::create_transfer_provider_router;


pub async fn create_core_provider_router(config: &CoreApplicationProviderConfig) -> Router {
    let app_config: ApplicationProviderConfig = config.clone().into();
    let auth_router = create_ssi_provider_router(app_config.clone().into()).await;
    let transfer_router = create_transfer_provider_router(&app_config.clone().into()).await;
    let cn_router = create_contract_negotiation_provider_router(&app_config.clone().into()).await;
    let catalog_router = create_catalog_router(&app_config.clone().into()).await;
    let mate_router = create_mate_consumer_router(app_config.clone().into()).await;

    Router::new()
        .merge(transfer_router)
        .merge(cn_router)
        .merge(catalog_router)
        .merge(auth_router)
        .merge(mate_router)
}
