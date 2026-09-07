/*
 * Copyright (C) 2026 - Universidad Politécnica de Madrid - UPM
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 */

use chrono::{DateTime, Utc};
use uuid::Uuid;
use ymir::errors::Outcome;

use crate::entities::role::RbacRole;
use crate::services::pat_service::views::{CreatePatResponse, PatView};
use common::auth::claims::Claims;

pub(crate) mod service;
pub mod views;

#[async_trait::async_trait]
pub(crate) trait PatServiceTrait: Send + Sync + 'static {
    async fn create_pat(
        &self,
        tenant_id: &str,
        name: &str,
        role: RbacRole,
        scopes: Vec<String>,
        expires_at: Option<DateTime<Utc>>,
    ) -> Outcome<CreatePatResponse>;

    async fn list_pats(&self, tenant_id: &str) -> Outcome<Vec<PatView>>;

    async fn revoke_pat(&self, tenant_id: &str, id: Uuid) -> Outcome<()>;

    async fn validate_pat(&self, raw_token: &str) -> Outcome<Claims>;
}
