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

use axum::http::Method;
use axum::Router;
use rainbow_catalog::http::idsa_api as catalog_ll_api_router;
use rainbow_catalog::http::rainbow_catalog_api as catalog_hl_api_router;
use rainbow_catalog::http::rainbow_policies_api as catalog_policies_api_router;
use rainbow_contracts::consumer::http::idsa_api as contracts_consumer_ll_api_router;
use rainbow_contracts::consumer::http::rainbow_cn_api as contracts_consumer_hl_api_router;
use rainbow_contracts::consumer::http::rainbow_idsa_triggers as contracts_consumer_triggers_api_router;
use rainbow_contracts::provider::http::idsa_api as contracts_provider_ll_api_router;
use rainbow_contracts::provider::http::rainbow_cn_api as contracts_provider_hl_api_router;
use rainbow_contracts::provider::http::rainbow_idsa_triggers as contracts_provider_triggers_api_router;

use rainbow_common::misc_router;
use rainbow_dataplane::proxy::provider_http;
use rainbow_transfer::consumer::http::hl_api as consumer_hl_api_router;
use rainbow_transfer::consumer::http::protocol_api as consumer_ll_api_router;

use rainbow_dataplane::proxy::consumer_http;
use rainbow_transfer::provider::http::hl_api as provider_hl_api_router;
use rainbow_transfer::provider::http::protocol_api as provider_ll_api_router;

use tower_http::cors::{Any, CorsLayer};

fn _create_cors_layer() -> CorsLayer {
    // TODO Cors in env
    CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_origin(Any)
}

pub async fn get_provider_routes() -> Router {
    Router::new()
        .merge(misc_router::router())
        .merge(provider_ll_api_router::router())
        .merge(provider_hl_api_router::router())
        .merge(provider_http::provider_dataplane_router())
        .merge(catalog_ll_api_router::catalog_router().await.unwrap())
        .merge(catalog_hl_api_router::catalog_api_router().await.unwrap())
        .merge(catalog_policies_api_router::catalog_policies_api_router().await.unwrap())
        .merge(contracts_provider_ll_api_router::router())
        .merge(contracts_provider_hl_api_router::router())
        .merge(contracts_provider_triggers_api_router::router())
        .layer(_create_cors_layer())
    // .layer(TraceLayer::new_for_http())
}

pub async fn get_consumer_routes() -> Router {
    Router::new()
        .merge(misc_router::router())
        .merge(consumer_ll_api_router::router())
        .merge(consumer_hl_api_router::router())
        .merge(consumer_http::consumer_dataplane_router())
        .merge(contracts_consumer_ll_api_router::router())
        .merge(contracts_consumer_hl_api_router::router())
        .merge(contracts_consumer_triggers_api_router::router())
        .layer(_create_cors_layer())
    // .layer(TraceLayer::new_for_http())
}
