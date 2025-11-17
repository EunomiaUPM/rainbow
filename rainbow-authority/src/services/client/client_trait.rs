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

use crate::types::enums::request::Body;
use axum::async_trait;
use axum::http::HeaderMap;
use reqwest::Response;

#[async_trait]
pub trait ClientServiceTrait: Send + Sync {
    async fn get(&self, url: &str, headers: Option<HeaderMap>) -> anyhow::Result<Response>;
    async fn post(&self, url: &str, headers: Option<HeaderMap>, body: Body) -> anyhow::Result<Response>;
    async fn put(&self, url: &str, headers: Option<HeaderMap>, body: Body) -> anyhow::Result<Response>;
    async fn delete(&self, url: &str, headers: Option<HeaderMap>, body: Body) -> anyhow::Result<Response>;
}
