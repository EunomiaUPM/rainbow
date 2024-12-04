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
use crate::consumer::http::api::__path_handle_get_all_callbacks;
use crate::consumer::http::api::handle_get_all_callbacks;
use axum::Router;
use utoipa::{OpenApi, PartialSchema};
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;
use utoipa_scalar::{Scalar, Servable};
use utoipa_swagger_ui::SwaggerUi;

// https://github.com/juhaku/utoipa/blob/master/examples/axum-utoipa-bindings/src/main.rs
#[derive(OpenApi)]
struct HighLevelConsumerApiDoc;

pub fn create_openapi_router() -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(handle_get_all_callbacks))
}

pub fn open_api_setup() -> anyhow::Result<Router> {
    let (router, api) = OpenApiRouter::with_openapi(HighLevelConsumerApiDoc::openapi())
        .nest("", create_openapi_router())
        .split_for_parts();

    let router = router
        .merge(SwaggerUi::new("/swagger-ui").url("/api/v1/openapi.json", api.clone()))
        .merge(Scalar::with_url("/api/v1", api));
    Ok(router)
}
