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

use crate::auth_consumer::entities::{auth, auth_verification, auth_interaction, prov, authority};
use axum::async_trait;
use rainbow_common::auth::gnap::grant_request::Interact4GR;
use sea_orm::DatabaseConnection;
use serde_json::Value;

pub mod sql;

pub trait AuthConsumerRepoFactory: AuthConsumerRepoTrait + Send + Sync + Clone + 'static {
    fn create_repo(db_connection: DatabaseConnection) -> Self
    where
        Self: Sized;
}

#[async_trait]
pub trait AuthConsumerRepoTrait {
    async fn get_all_auths(&self, limit: Option<u64>, offset: Option<u64>) -> anyhow::Result<Vec<auth::Model>>;

    async fn get_auth_by_id(&self, id: String) -> anyhow::Result<auth::Model>;

    async fn create_auth(
        &self,
        id: String,
        uri: String,
        provider_id: String,
        provider_slug: String,
        actions: String,
        interact: Interact4GR,
    ) -> anyhow::Result<auth::Model>;

    async fn auth_pending(&self, id: String, assigned_id: String, continue_uri: String, as_nonce: String) -> anyhow::Result<auth::Model>;

    async fn delete_auth(&self, id: String) -> anyhow::Result<auth::Model>;

    async fn get_interaction_by_id(&self, id: String) -> anyhow::Result<auth_interaction::Model>;

    async fn update_interaction_by_id(
        &self,
        id: String,
        interact_ref: String,
        hash: String,
    ) -> anyhow::Result<auth_interaction::Model>;

    async fn create_auth_verification(&self, id: String, uri: String) -> anyhow::Result<auth_verification::Model>;
    async fn grant_req_approved(&self, id: String, jwt: String) -> anyhow::Result<auth::Model>;
    async fn create_prov(&self, provider: String, provider_route: String) -> anyhow::Result<()>;
    async fn prov_onboard(&self, provider: String) -> anyhow::Result<()>;
    async fn get_all_provs(&self) -> anyhow::Result<Vec<prov::Model>>;
    async fn get_prov(&self, provider: String) -> anyhow::Result<prov::Model>;
}


#[async_trait]
pub trait ParticipantRepoTrait {
    // async fn create_process(&self, id: Option<String>, authority: String, assigned_id: String, grant_endpoint: String) -> anyhow::Result<authority::Model>;
}