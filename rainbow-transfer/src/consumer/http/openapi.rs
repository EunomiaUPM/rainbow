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

use utoipa::openapi::OpenApi as OpenApiModel;
use utoipa::{OpenApi, PartialSchema};
use utoipa_redoc::{Redoc, Servable};

// https://github.com/juhaku/utoipa/blob/master/examples/axum-utoipa-bindings/src/main.rs
#[derive(OpenApi)]
struct HighLevelConsumerApiDoc;

pub fn open_api_setup() -> anyhow::Result<Redoc<OpenApiModel>> {
    let api_docs = HighLevelConsumerApiDoc::openapi();
    let router = Redoc::with_url("/api/v1", api_docs);
    Ok(router)
}
