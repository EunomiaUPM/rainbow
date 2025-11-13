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
use super::super::ClientServiceTrait;
use crate::ssi::common::types::enums::request::Body;
use anyhow::bail;
use axum::async_trait;
use axum::http::HeaderMap;
use rainbow_common::errors::{CommonErrors, ErrorLog};
use reqwest::{Client, RequestBuilder, Response};
use serde_json::{to_string, Value};
use std::time::Duration;
use tracing::error;

pub struct BasicClientService {
    client: Client,
}

impl BasicClientService {
    pub fn new() -> Self {
        Self {
            client: Client::builder().timeout(Duration::from_secs(10)).build().expect("Failed to build request client"),
        }
    }
    async fn send_request(&self, req: RequestBuilder, method: &str, url: &str) -> anyhow::Result<Response> {
        match req.send().await {
            Ok(resp) => Ok(resp),
            Err(e) => {
                let http_code = e.status().map(|s| s.as_u16());
                let error = CommonErrors::petition_new(
                    url,
                    method,
                    http_code,
                    &e.to_string(),
                );
                error!("{}", error.log());
                bail!(error.log());
            }
        }
    }
    fn apply_body(&self, req: RequestBuilder, body: Body) -> RequestBuilder {
        match body {
            Body::Json(value) => req.json(&value),
            Body::Raw(s) => req.body(s),
            Body::None => req,
        }
    }
}

#[async_trait]
impl ClientServiceTrait for BasicClientService {
    async fn get(&self, url: &str, headers: Option<HeaderMap>) -> anyhow::Result<Response> {
        let req = if let Some(h) = headers { self.client.get(url).headers(h) } else { self.client.get(url) };
        self.send_request(req, "GET", url).await
    }

    async fn post(&self, url: &str, headers: Option<HeaderMap>, body: Body) -> anyhow::Result<Response> {
        let req = if let Some(h) = headers { self.client.post(url).headers(h) } else { self.client.post(url) };
        let req = self.apply_body(req, body);
        self.send_request(req, "POST", url).await
    }

    async fn put(&self, url: &str, headers: Option<HeaderMap>, body: Body) -> anyhow::Result<Response> {
        let req = if let Some(h) = headers { self.client.put(url).headers(h) } else { self.client.put(url) };
        let req = self.apply_body(req, body);
        self.send_request(req, "PUT", url).await
    }

    async fn delete(&self, url: &str, headers: Option<HeaderMap>, body: Body) -> anyhow::Result<Response> {
        let req = if let Some(h) = headers { self.client.delete(url).headers(h) } else { self.client.delete(url) };
        let req = self.apply_body(req, body);
        self.send_request(req, "DELETE", url).await
    }
}
