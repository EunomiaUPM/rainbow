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
use crate::ssi_auth::consumer::core::{consumer_vc_request, ConsumerSSIVCRequest};
use crate::ssi_auth::consumer::session::SESSION_MANAGER;
use anyhow::bail;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};
use axum::http::{Method, Uri};
use rainbow_common::err::transfer_err::TransferErrorType;
use reqwest::{StatusCode, Url};
use serde_json::Value;
use tracing::info;
use urlencoding::decode;

pub fn router() -> Router {
    Router::new()
        .route("/log", post(login))// TESTING PURPOSES
        .route("/url", post(url))// TESTING PURPOSES
        .route("/matcheo", post(matcheo))// TESTING PURPOSES
        .route("/exchange", post(exchange))// TESTING PURPOSES
        .route("/auth/ssi", post(authssi))
        .fallback(fallback)
}

async fn authssi() -> impl IntoResponse {

    StatusCode::OK
}


// ---------------------------------------------------------------------







async fn login() -> impl IntoResponse {
    let mut session = SESSION_MANAGER.lock().await;

    match session.access().await {
        Ok(()) => {
            info!("Sesion creada con éxito");
            Json("Login successful")
        }
        Err(err) => {
            info!("FRACASO");
            Json("FRACASO")
        }
    }
}

async fn url(Json(payload): Json<MessageRequest>) -> impl IntoResponse {
    let mut session = SESSION_MANAGER.lock().await;

    match session.access().await {
        Ok(()) => {
            info!("Sesion creada con éxito");
        }
        Err(err) => {
            info!("FRACASO");
        }
    }

    match session.joinexchange(payload.message).await {
        Ok(string) => {
            println!();
            println!("UURRLL: {}", string);
            println!()
        }
        Err(err) => println!("errror: {}", err),
    }

    StatusCode::OK
}

async fn matcheo(Json(payload): Json<MessageRequest>) -> impl IntoResponse {
    let mut session = SESSION_MANAGER.lock().await;

    match session.access().await {
        Ok(()) => {
            info!("Sesion creada con éxito");
        }
        Err(err) => {
            info!("FRACASO");
        }
    }

    let res =
        session.joinexchange(payload.message).await.unwrap_or_else(|err| String::from("ERROR"));
    let url = Url::parse(decode(&res).unwrap().as_ref()).unwrap();

    if let Some((_, vpd)) = url.query_pairs().find(|(key, _)| key == "presentation_definition") {
        println!("vp: {}", vpd);

        match serde_json::from_str::<Value>(&vpd) {
            Ok(json) => {
                println!("json: {:?}", json);
            }
            Err(err) => {
                println!("ERROR")
            }
        }
    } else {
        println!("ERROR");
    }

    StatusCode::OK
}

async fn exchange(Json(payload): Json<MessageRequest>) -> impl IntoResponse {
    let mut session = SESSION_MANAGER.lock().await;

    match session.access().await {
        Ok(()) => {
            info!("Sesion creada con éxito");
        }
        Err(err) => {
            info!("FRACASO");
        }
    }

    let res =
        session.joinexchange(payload.message).await.unwrap_or_else(|err| String::from("ERROR"));
    println!();
    println!("UURRLL1: {}", res);
    println!();

    let url = Url::parse(decode(&res).unwrap().as_ref()).unwrap();

    println!();
    println!("UURRLL2: {}", url);
    println!();

    if let Some((_, vpd)) = url.query_pairs().find(|(key, _)| key == "presentation_definition") {
        let vpd = serde_json::from_str::<Value>(&vpd).unwrap();

        let vcs = session.match_vc4vp(vpd).await.unwrap();

        let mut creds = Vec::new();
        for vc in vcs {
            creds.push(vc.id);
        }

        let kk = session.present_vp(res, creds).await.unwrap();
    } else {
        println!("ERROR");
    }

    StatusCode::OK
}

async fn fallback(method: Method, uri: Uri) -> (StatusCode, String) {
    let kk = format!("{} {}", method, uri);
    info!("{}", kk);
    (StatusCode::NOT_FOUND, format!("No route for {uri}"))
}

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct MessageRequest {
    message: String,
}
