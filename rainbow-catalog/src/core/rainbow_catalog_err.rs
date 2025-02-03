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

use axum::Json;
use rainbow_db::catalog::repo::CatalogRepoErrors;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use urn::Urn;

#[derive(Error, Debug)]
pub enum CatalogError {
    #[error("{entity} with id {id:?} not found")]
    NotFound { id: Urn, entity: String },
    #[error("Error from database: {0}")]
    DbErr(CatalogRepoErrors),
    #[error("Conversion Error: {0}")]
    ConversionError(anyhow::Error),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CatalogErrorOut {
    pub error: CatalogErrorOutDetail,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct CatalogErrorOutDetail {
    pub code: String,
    pub title: String,
    pub message: String,
}

impl CatalogErrorOut {
    pub fn new(code: String, title: String, message: String) -> Self {
        CatalogErrorOut { error: CatalogErrorOutDetail { code, title, message } }
    }
}

impl Into<Json<CatalogErrorOut>> for CatalogError {
    fn into(self) -> Json<CatalogErrorOut> {
        match self {
            CatalogError::NotFound { id, entity } => Json(CatalogErrorOut::new(
                "404".to_string(),
                "NOT_FOUND".to_string(),
                "Not Found".to_string(),
            )),
            CatalogError::DbErr(e) => Json(CatalogErrorOut::new(
                "500".to_string(),
                "INTERNAL_SERVER_ERROR".to_string(),
                e.to_string(),
            )),
            CatalogError::ConversionError(e) => Json(CatalogErrorOut::new(
                "500".to_string(),
                "INTERNAL_SERVER_ERROR".to_string(),
                e.to_string(),
            )),
        }
    }
}
