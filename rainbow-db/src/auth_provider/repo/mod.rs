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

use crate::auth_provider::entities::{auth, auth_interaction, auth_verification};
use anyhow::Error;
use axum::async_trait;
use rainbow_common::auth::gnap::grant_request::Interact4GR;
use sea_orm::DatabaseConnection;
use thiserror::Error;

pub mod sql;

pub trait AuthProviderRepoFactory: AuthProviderRepoTrait + Send + Sync + Clone + 'static {
    fn create_repo(db_connection: DatabaseConnection) -> Self
    where
        Self: Sized;
}

#[async_trait]
pub trait AuthProviderRepoTrait {
    async fn get_all_auths(
        &self,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> anyhow::Result<Vec<auth::Model>, AuthProviderRepoErrors>;
    async fn get_auth_by_id(&self, id: String) -> anyhow::Result<auth::Model, AuthProviderRepoErrors>;
    async fn create_auth(
        &self,
        consumer: String,
        audience: String,
        grant_uri: String,
        actions: String,
        interact: Interact4GR,
    ) -> anyhow::Result<
        (
            auth::Model,
            auth_interaction::Model,
            auth_verification::Model,
        ),
        AuthProviderRepoErrors,
    >;
    async fn create_truncated_auth(
        &self,
        audience: String,
        state: String,
    ) -> anyhow::Result<(auth::Model, auth_verification::Model), AuthProviderRepoErrors>;
    async fn update_auth_status(
        &self,
        id: String,
        status: String,
        end: bool,
    ) -> anyhow::Result<auth::Model, AuthProviderRepoErrors>;
    async fn delete_auth(&self, id: String) -> anyhow::Result<auth::Model, AuthProviderRepoErrors>;

    async fn get_interaction_by_id(
        &self,
        id: String,
    ) -> anyhow::Result<auth_interaction::Model, AuthProviderRepoErrors>;

    async fn get_auth_by_state(
        &self,
        state: String,
    ) -> anyhow::Result<auth_verification::Model, AuthProviderRepoErrors>;

    async fn get_av_by_id_update_holder(
        &self,
        id: String,
        vpt: String,
        holder: String,
    ) -> anyhow::Result<auth_verification::Model, AuthProviderRepoErrors>;

    async fn update_verification_result(
        &self,
        id: String,
        result: bool,
    ) -> anyhow::Result<auth_verification::Model, AuthProviderRepoErrors>;

    async fn save_token(
        &self,
        id: String,
        base_url: String,
        token: String,
    ) -> anyhow::Result<auth::Model, AuthProviderRepoErrors>;

    async fn get_auth_by_interact_ref(
        &self,
        interact_ref: String,
    ) -> anyhow::Result<auth_interaction::Model, AuthProviderRepoErrors>;
    async fn is_token_in_db(&self, token: String) -> anyhow::Result<bool, AuthProviderRepoErrors>;
    async fn get_auth_ver_by_id(&self, id: String) -> anyhow::Result<auth_verification::Model, AuthProviderRepoErrors>;
}

#[derive(Debug, Error)]
pub enum AuthProviderRepoErrors {
    #[error("Auth not found")]
    AuthNotFound,
    #[error("Auth interaction not found")]
    AuthInteractionNotFound,
    #[error("Auth verification not found")]
    AuthVerificationNotFound,

    #[error("Error fetching auth. {0}")]
    ErrorFetchingAuth(Error),
    #[error("Error creating auth. {0}")]
    ErrorCreatingAuth(Error),
    #[error("Error deleting auth. {0}")]
    ErrorDeletingAuth(Error),
    #[error("Error updating auth. {0}")]
    ErrorUpdatingAuth(Error),

    #[error("Error fetching auth interaction. {0}")]
    ErrorFetchingAuthInteraction(Error),
    #[error("Error creating auth interaction. {0}")]
    ErrorCreatingAuthInteraction(Error),
    #[error("Error deleting auth interaction. {0}")]
    ErrorDeletingAuthInteraction(Error),
    #[error("Error updating auth interaction. {0}")]
    ErrorUpdatingAuthInteraction(Error),

    #[error("Error fetching auth verification. {0}")]
    ErrorFetchingAuthVerification(Error),
    #[error("Error creating auth verification. {0}")]
    ErrorCreatingAuthVerification(Error),
    #[error("Error deleting auth verification. {0}")]
    ErrorDeletingAuthVerification(Error),
    #[error("Error updating auth verification. {0}")]
    ErrorUpdatingAuthVerification(Error),
}
