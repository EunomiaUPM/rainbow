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

use axum::body::Body;
use axum::response::Response;
use reqwest::Response as ReqwestResponse;
pub async fn forward_response(reqwest_response: ReqwestResponse) -> Response {
    let status = reqwest_response.status();
    let headers = reqwest_response.headers().clone();
    let body_stream = reqwest_response.bytes_stream();
    let body = Body::from_stream(body_stream);
    let mut response = Response::builder().status(status);
    let response_headers = response.headers_mut().unwrap();
    for (key, value) in headers.iter() {
        response_headers.insert(key, value.clone());
    }

    response.body(body).unwrap()
}
