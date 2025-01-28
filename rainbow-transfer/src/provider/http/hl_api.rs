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

use crate::provider::lib::api::{
    delete_agreement, get_agreement_by_id, get_all_agreements, get_all_transfers,
    get_messages_by_id, get_messages_by_transfer, get_transfer_by_id, post_agreement,
    put_agreement, EditAgreement, NewAgreement,
};
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{delete, get, post, put};
use axum::{Json, Router};
use log::info;
use rainbow_common::utils::get_urn_from_string;
use serde_json::json;

pub fn router() -> Router {
    Router::new()
        .route("/api/v1/transfers", get(handle_get_all_transfers))
        .route("/api/v1/transfers/:id", get(handle_get_transfer_by_id))
        .route(
            "/api/v1/transfers/:id/messages",
            get(handle_get_messages_by_transfer),
        )
        .route(
            "/api/v1/transfers/:id/messages/:mid",
            get(handle_get_messages_by_id),
        )
        .route("/api/v1/agreements", get(handle_get_all_agreements))
        .route("/api/v1/agreements/:id", get(handle_get_agreement_by_id))
        .route("/api/v1/agreements", post(handle_post_agreement))
        .route("/api/v1/agreements/:id", put(handle_put_agreement))
        .route("/api/v1/agreements/:id", delete(handle_delete_agreement))
    // TODO HL api to handle messages to modify state
}

async fn handle_get_all_transfers() -> impl IntoResponse {
    info!("GET /api/v1/transfers");

    let transfers = get_all_transfers().await;
    if transfers.is_err() {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }
    (StatusCode::OK, Json(transfers.unwrap())).into_response()
}

async fn handle_get_transfer_by_id(Path(id): Path<String>) -> impl IntoResponse {
    info!("GET /api/v1/transfers/{}", id.to_string());
    let id = get_urn_from_string(&id).unwrap();
    match get_transfer_by_id(id).await {
        Ok(transfer) => match transfer {
            Some(transfer) => (StatusCode::OK, Json(transfer)).into_response(),
            None => (StatusCode::NOT_FOUND).into_response(),
        },
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

async fn handle_get_messages_by_transfer(Path(id): Path<String>) -> impl IntoResponse {
    info!("GET /api/v1/transfers/{}/messages", id.to_string());
    let id = get_urn_from_string(&id).unwrap();
    match get_messages_by_transfer(id).await {
        Ok(messages) => (StatusCode::OK, Json(messages)).into_response(),
        Err(e) => (StatusCode::OK, e.to_string()).into_response(),
    }
}

async fn handle_get_messages_by_id(Path((id, mid)): Path<(String, String)>) -> impl IntoResponse {
    info!("GET /api/v1/agreements/{}/messages/{}", id, mid);
    let id = get_urn_from_string(&id).unwrap();
    let mid = get_urn_from_string(&mid).unwrap();

    match get_messages_by_id(id, mid).await {
        Ok(messages) => (StatusCode::OK, Json(messages)).into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    }
}

async fn handle_get_all_agreements() -> impl IntoResponse {
    info!("GET /api/v1/agreements/");

    match get_all_agreements().await {
        Ok(agreements) => (StatusCode::OK, Json(agreements)).into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    }
}

async fn handle_get_agreement_by_id(Path(id): Path<String>) -> impl IntoResponse {
    info!("GET /api/v1/agreements/{}", id);
    let id = get_urn_from_string(&id).unwrap();

    match get_agreement_by_id(id).await {
        Ok(agreement) => (StatusCode::OK, Json(agreement)).into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    }
}

async fn handle_post_agreement(Json(input): Json<NewAgreement>) -> impl IntoResponse {
    info!("POST /api/v1/agreements/");
    match post_agreement(input).await {
        Ok(agreement) => (StatusCode::CREATED, Json(agreement)).into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    }
}

async fn handle_put_agreement(
    Path(id): Path<String>,
    Json(input): Json<EditAgreement>,
) -> impl IntoResponse {
    info!("PUT /api/v1/agreements/{}", id);
    let id = get_urn_from_string(&id).unwrap();

    match put_agreement(id, input).await {
        Ok(agreement) => (StatusCode::CREATED, Json(agreement)).into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    }
}

async fn handle_delete_agreement(Path(id): Path<String>) -> impl IntoResponse {
    info!("DELETE /api/v1/agreements/{}", id);
    let id = get_urn_from_string(&id).unwrap();

    match delete_agreement(id).await {
        Ok(_) => (StatusCode::ACCEPTED).into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    }
}
