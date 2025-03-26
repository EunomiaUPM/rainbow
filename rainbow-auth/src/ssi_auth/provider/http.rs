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


use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};
use rainbow_common::err::transfer_err::TransferErrorType;
use reqwest::StatusCode;
use tracing::info;
use crate::ssi_auth::provider::manager::Manager;

pub fn create_ssi_auth_router() -> Router {
    Router::new()
        .route("/petition", post(handle_petition()))
        .route("/vpexchange", post(vpexchange()))
        .route("/vpdefinition", post(vpdefinition()))
        .route("/presentation", post(presentation()))

}

fn handle_petition() -> impl IntoResponse {
    info!("POST /petition");

    let uri = Manager::generate_exchange_uri();
    Json(uri)
}

fn vpexchange() -> impl IntoResponse {
    info!("POST /vpexchange");

}

fn vpdefinition() -> impl IntoResponse {
    info!("POST /vpdefinition");

}

fn presentation() -> impl IntoResponse {
    info!("POST /presentation");

}