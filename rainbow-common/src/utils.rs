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
use crate::errors::helpers::BadFormat;
use crate::errors::{CommonErrors, ErrorLog};
use anyhow::bail;
use axum::extract::rejection::JsonRejection;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fs;
use tracing::{error, info};
use urn::Urn;
use uuid::Uuid;

static UUID_PREFIX: &str = "urn:uuid:";

pub fn get_urn(optional_urn: Option<Urn>) -> Urn {
    optional_urn.unwrap_or_else(|| {
        let uuid = Uuid::new_v4();
        let id_string = format!("{}{}", UUID_PREFIX, uuid);
        let urn = id_string.parse::<Urn>().unwrap();
        urn
    })
}

pub fn get_urn_from_string(string_in: &String) -> anyhow::Result<Urn> {
    let urn_res = string_in.parse::<Urn>();
    match urn_res {
        Ok(urn_res) => Ok(urn_res),
        Err(e) => {
            let e_ = CommonErrors::format_new(BadFormat::Received, &e.to_string());
            error!("{}", e_.log());
            bail!(e_);
        }
    }
}

pub async fn server_status() -> impl IntoResponse {
    info!("Someone checked server status");
    (StatusCode::OK, "Server is Okay!").into_response()
}

pub fn get_from_opt<T>(value: &Option<T>, field_name: &str) -> anyhow::Result<T>
where
    T: Clone + Serialize + DeserializeOwned,
{
    match value {
        Some(v) => Ok(v.clone()),
        None => {
            let error = CommonErrors::format_new(
                BadFormat::Received,
                &format!("Missing field: {}", field_name),
            );
            error!("{}", error.log());
            bail!(error);
        }
    }
}

pub fn read(path: &str) -> anyhow::Result<String> {
    match fs::read_to_string(&path) {
        Ok(data) => Ok(data),
        Err(e) => {
            let error = CommonErrors::read_new(path, &e.to_string());
            error!("{}", error.log());
            bail!(error)
        }
    }
}
