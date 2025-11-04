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

use axum::extract::Request;
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::Response;
use std::sync::Arc;
use tracing::debug;

#[derive(Debug, Clone)]
pub struct RequestInfo {
    pub token: String,
}

pub async fn extract_request_info(mut request: Request, next: Next) -> Result<Response, StatusCode> {
    debug!("Request info headers middleware");
    // 1. Extract headers
    let headers = request.headers();
    let token = headers
        .get("Authorization")
        .and_then(|value| value.to_str().ok())
        .map(String::from)
        .unwrap_or_else(|| "".to_string())
        .replace("Bearer ", "");
    // 2. Setup struct
    let request_info = RequestInfo { token };
    // 3. Insert into extensions
    request.extensions_mut().insert(Arc::new(request_info));
    // 4. Bye
    Ok(next.run(request).await)
}
