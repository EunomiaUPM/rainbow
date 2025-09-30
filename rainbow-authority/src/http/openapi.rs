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
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse};
use axum::Router;
use axum::routing::get;
use once_cell::sync::Lazy;

pub fn route_openapi() -> Router {
    let openapi_spec = "/api/v1/auth/openapi.json";
    Router::new()
        .route(openapi_spec, get(get_open_api))
        .route("/api/v1/auth/openapi", get(|| swagger_ui_html(openapi_spec)),
        )
}

static OPENAPI_JSON: Lazy<&'static str> =
    Lazy::new(|| include_str!("./../../../static/specs/openapi/authority/authority.json"));

async fn get_open_api() -> impl IntoResponse {
    (
        StatusCode::OK,
        [("Content-Type", "application/json")],
        OPENAPI_JSON.as_bytes(),
    )
        .into_response()
}



async fn swagger_ui_html(route: &str) -> impl IntoResponse {
    let html = format!(
        r#"<!doctype html>
<html>
<head>
  <meta charset="utf-8" />
  <title>Swagger UI</title>
  <link rel="stylesheet" href="https://unpkg.com/swagger-ui-dist/swagger-ui.css" />
  <style>
    body {{ margin:0; display:flex; flex-direction:column; height:100vh; }}
    #swagger-ui {{ flex:1; }}
    footer {{
      text-align:center;
      padding:10px;
      background:#f8f8f8;
      border-top:1px solid #ddd;
      font-size:0.9em;
      color:#555;
    }}
  </style>
</head>
<body>
  <div id="swagger-ui"></div>
  <footer>
    &copy; 2025 Universidad Politécnica de Madrid - UPM | Rainbow API Documentation
  </footer>
  <script src="https://unpkg.com/swagger-ui-dist/swagger-ui-bundle.js"></script>
  <script>
    window.onload = function() {{
      SwaggerUIBundle({{
        url: "{}",
        dom_id: '#swagger-ui',
      }});
    }};
  </script>
</body>
</html>"#,
        route
    );

    Html(html)
}
