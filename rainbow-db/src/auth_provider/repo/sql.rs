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

use crate::auth_provider::entities::auth;
use crate::auth_provider::entities::auth_interaction;
use crate::auth_provider::entities::auth_verification;
use crate::auth_provider::repo::{AuthProviderRepoErrors, AuthProviderRepoFactory, AuthProviderRepoTrait};
use axum::async_trait;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use chrono;
use rainbow_common::auth::gnap::grant_request::Interact4GR;
use rand::{distributions::Alphanumeric, Rng};
use sea_orm::sqlx::types::uuid;
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter,
    QuerySelect,
};
use serde_json::Value;
use sha2::{Digest, Sha256};

#[derive(Clone)]
pub struct AuthProviderRepoForSql {
    db_connection: DatabaseConnection,
}

impl AuthProviderRepoForSql {
    fn new(db_connection: DatabaseConnection) -> Self {
        Self { db_connection }
    }
}

impl AuthProviderRepoFactory for AuthProviderRepoForSql {
    fn create_repo(db_connection: DatabaseConnection) -> Self
    where
        Self: Sized,
    {
        Self::new(db_connection)
    }
}

#[async_trait]
impl AuthProviderRepoTrait for AuthProviderRepoForSql {
    async fn get_all_auths(
        &self,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> anyhow::Result<Vec<auth::Model>, AuthProviderRepoErrors> {
        let auths = auth::Entity::find()
            .limit(limit.unwrap_or(100000))
            .offset(offset.unwrap_or(0))
            .all(&self.db_connection)
            .await
            .map_err(|e| AuthProviderRepoErrors::ErrorFetchingAuth(e.into()))?;
        Ok(auths)
    }

    async fn get_auth_by_id(&self, auth_id: String) -> anyhow::Result<auth::Model, AuthProviderRepoErrors> {
        let auth = auth::Entity::find_by_id(auth_id.clone())
            .one(&self.db_connection)
            .await
            .map_err(|e| AuthProviderRepoErrors::ErrorFetchingAuth(e.into()))?
            .ok_or(AuthProviderRepoErrors::AuthNotFound)?;
        Ok(auth)
    }

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
    > {
        // create variables
        let id = uuid::Uuid::new_v4().to_string();
        let state: String = rand::thread_rng().sample_iter(&Alphanumeric).take(12).map(char::from).collect();
        let as_nonce: String = rand::thread_rng().sample_iter(&Alphanumeric).take(36).map(char::from).collect();
        let nonce: String = rand::thread_rng().sample_iter(&Alphanumeric).take(12).map(char::from).collect();
        let interact_ref: String = rand::thread_rng().sample_iter(&Alphanumeric).take(16).map(char::from).collect();

        let start: Value = Value::String(
            serde_json::to_string(&interact.start).map_err(|e| AuthProviderRepoErrors::ErrorCreatingAuth(e.into()))?,
        );
        let hash_method = interact.finish.hash_method.unwrap_or_else(|| "sha-256".to_string());
        let hash_input = format!(
            "{}\n{}\n{}\n{}",
            interact.finish.nonce, as_nonce, interact_ref, grant_uri
        );

        let mut hasher = Sha256::new();
        hasher.update(hash_input.as_bytes());
        let result = hasher.finalize();
        let hash = URL_SAFE_NO_PAD.encode(result);

        // create models
        let auth_model = auth::ActiveModel {
            id: ActiveValue::Set(id.clone()),
            consumer: ActiveValue::Set(Some(consumer)),
            actions: ActiveValue::Set(actions),
            status: ActiveValue::Set("Pending".to_string()),
            token: ActiveValue::Set(None),
            created_at: ActiveValue::Set(chrono::Utc::now().naive_utc()),
            ended_at: ActiveValue::Set(None),
            ..Default::default()
        };
        let auth_interaction_model = auth_interaction::ActiveModel {
            id: ActiveValue::Set(id.clone()),
            start: ActiveValue::Set(start),
            method: ActiveValue::Set(interact.finish.method),
            uri: ActiveValue::Set(interact.finish.uri),
            client_nonce: ActiveValue::Set(interact.finish.nonce),
            as_nonce: ActiveValue::Set(as_nonce),
            interact_ref: ActiveValue::Set(interact_ref),
            grant_endpoint: ActiveValue::Set(grant_uri),
            hash: ActiveValue::Set(hash),
            hash_method: ActiveValue::Set(Some(hash_method)),
            hints: ActiveValue::Set(None), // TODO
            ..Default::default()
        };
        let auth_verification_model = auth_verification::ActiveModel {
            id: ActiveValue::Set(id.clone()),
            state: ActiveValue::Set(state.clone()),
            nonce: ActiveValue::Set(nonce),
            audience: ActiveValue::Set(audience + "/" + state.as_str()),
            holder: ActiveValue::Set(None),
            vpt: ActiveValue::Set(None),
            success: ActiveValue::Set(None),
            status: ActiveValue::Set("Pending".to_string()),
            created_at: ActiveValue::Set(chrono::Utc::now().naive_utc()),
            ended_at: ActiveValue::Set(None),
        };

        // persist models
        let auth = auth::Entity::insert(auth_model)
            .exec_with_returning(&self.db_connection)
            .await
            .map_err(|e| AuthProviderRepoErrors::ErrorCreatingAuth(e.into()))?;

        let auth_interaction = auth_interaction::Entity::insert(auth_interaction_model)
            .exec_with_returning(&self.db_connection)
            .await
            .map_err(|e| AuthProviderRepoErrors::ErrorCreatingAuthInteraction(e.into()))?;

        let auth_verification = auth_verification::Entity::insert(auth_verification_model)
            .exec_with_returning(&self.db_connection)
            .await
            .map_err(|e| AuthProviderRepoErrors::ErrorCreatingAuthVerification(e.into()))?;

        Ok((auth, auth_interaction, auth_verification))
    }

    async fn create_truncated_auth(
        &self,
        audience: String,
        state: String,
    ) -> anyhow::Result<(auth::Model, auth_verification::Model), AuthProviderRepoErrors> {
        // create vars
        let id = uuid::Uuid::new_v4().to_string();
        let as_nonce: String = rand::thread_rng().sample_iter(&Alphanumeric).take(36).map(char::from).collect();
        let nonce: String = rand::thread_rng().sample_iter(&Alphanumeric).take(12).map(char::from).collect();
        let interact_ref: String = rand::thread_rng().sample_iter(&Alphanumeric).take(16).map(char::from).collect();

        let auth_model = auth::ActiveModel {
            id: ActiveValue::Set(id.clone()),
            consumer: ActiveValue::Set(None),
            actions: ActiveValue::Set("talk".to_string()),
            status: ActiveValue::Set("Pending".to_string()),
            token: ActiveValue::Set(None),
            created_at: ActiveValue::Set(chrono::Utc::now().naive_utc()),
            ended_at: ActiveValue::Set(None),
        };

        let auth_verification_model = auth_verification::ActiveModel {
            id: ActiveValue::Set(id.clone()),
            state: ActiveValue::Set(state.clone()),
            nonce: ActiveValue::Set(nonce),
            audience: ActiveValue::Set(audience + "/" + state.as_str()),
            holder: ActiveValue::Set(None),
            vpt: ActiveValue::Set(None),
            success: ActiveValue::Set(None),
            status: ActiveValue::Set("Pending".to_string()),
            created_at: ActiveValue::Set(chrono::Utc::now().naive_utc()),
            ended_at: ActiveValue::Set(None),
        };

        // persist models
        let auth = auth::Entity::insert(auth_model)
            .exec_with_returning(&self.db_connection)
            .await
            .map_err(|e| AuthProviderRepoErrors::ErrorCreatingAuth(e.into()))?;

        let auth_verification = auth_verification::Entity::insert(auth_verification_model)
            .exec_with_returning(&self.db_connection)
            .await
            .map_err(|e| AuthProviderRepoErrors::ErrorCreatingAuthVerification(e.into()))?;

        Ok((auth, auth_verification))
    }

    async fn update_auth_status(
        &self,
        id: String,
        status: String,
        end: bool,
    ) -> anyhow::Result<auth::Model, AuthProviderRepoErrors> {
        let mut entry = auth::Entity::find_by_id(id.clone())
            .one(&self.db_connection)
            .await
            .map_err(|e| AuthProviderRepoErrors::ErrorFetchingAuth(e.into()))?
            .ok_or(AuthProviderRepoErrors::AuthNotFound)?
            .into_active_model();

        entry.status = ActiveValue::Set(status);
        if end {
            entry.ended_at = ActiveValue::Set(Some(chrono::Utc::now().naive_utc()));
        }
        let upd_entry = entry.update(&self.db_connection).await;
        match upd_entry {
            Ok(upd_entry) => Ok(upd_entry),
            Err(e) => Err(AuthProviderRepoErrors::ErrorFetchingAuth(e.into())),
        }
    }

    async fn delete_auth(&self, id: String) -> anyhow::Result<auth::Model, AuthProviderRepoErrors> {
        let mut entry = auth::Entity::find_by_id(id.clone())
            .one(&self.db_connection)
            .await
            .map_err(|e| AuthProviderRepoErrors::ErrorFetchingAuth(e.into()))?
            .ok_or(AuthProviderRepoErrors::AuthNotFound)?;
        let ret = entry.clone();
        let active_model = entry.into_active_model();
        let _ = active_model
            .delete(&self.db_connection)
            .await
            .map_err(|e| AuthProviderRepoErrors::ErrorDeletingAuth(e.into()))?;
        Ok(ret)
    }

    async fn get_interaction_by_id(
        &self,
        id: String,
    ) -> anyhow::Result<auth_interaction::Model, AuthProviderRepoErrors> {
        let auth_interaction = auth_interaction::Entity::find_by_id(id.clone())
            .one(&self.db_connection)
            .await
            .map_err(|e| AuthProviderRepoErrors::ErrorFetchingAuthInteraction(e.into()))?
            .ok_or(AuthProviderRepoErrors::AuthInteractionNotFound)?;
        Ok(auth_interaction)
    }

    async fn get_auth_by_state(
        &self,
        state: String,
    ) -> anyhow::Result<auth_verification::Model, AuthProviderRepoErrors> {
        let auth = auth_verification::Entity::find()
            .filter(auth_verification::Column::State.eq(state.clone()))
            .one(&self.db_connection)
            .await
            .map_err(|e| AuthProviderRepoErrors::ErrorFetchingAuthVerification(e.into()))?
            .ok_or(AuthProviderRepoErrors::AuthVerificationNotFound)?;
        Ok(auth)
    }

    async fn get_av_by_id_update_holder(
        &self,
        id: String,
        vpt: String,
        holder: String,
    ) -> anyhow::Result<auth_verification::Model, AuthProviderRepoErrors> {
        let auth_ver = auth_verification::Entity::find_by_id(&id)
            .one(&self.db_connection)
            .await
            .map_err(|e| AuthProviderRepoErrors::ErrorFetchingAuthVerification(e.into()))?
            .ok_or(AuthProviderRepoErrors::AuthVerificationNotFound)?;

        let mut entry = auth_ver.into_active_model();
        entry.holder = ActiveValue::Set(Some(holder));
        entry.vpt = ActiveValue::Set(Some(vpt));

        let upd_entry = entry
            .update(&self.db_connection)
            .await
            .map_err(|e| AuthProviderRepoErrors::ErrorUpdatingAuthVerification(e.into()))?;
        Ok(upd_entry)
    }

    async fn update_verification_result(
        &self,
        id: String,
        result: bool,
    ) -> anyhow::Result<auth_verification::Model, AuthProviderRepoErrors> {
        let auth_ver = auth_verification::Entity::find_by_id(&id)
            .one(&self.db_connection)
            .await
            .map_err(|e| AuthProviderRepoErrors::ErrorFetchingAuthVerification(e.into()))?
            .ok_or(AuthProviderRepoErrors::AuthVerificationNotFound)?;

        let auth = auth::Entity::find_by_id(&id)
            .one(&self.db_connection)
            .await
            .map_err(|e| AuthProviderRepoErrors::ErrorFetchingAuth(e.into()))?
            .ok_or(AuthProviderRepoErrors::AuthNotFound)?;

        let mut ver_entry = auth_ver.into_active_model();
        let mut auth_entry = auth.into_active_model();
        ver_entry.success = ActiveValue::Set(Some(result));
        ver_entry.ended_at = ActiveValue::Set(Some(chrono::Utc::now().naive_utc()));
        if result {
            auth_entry.status = ActiveValue::Set("Processing".to_string());
        } else {
            auth_entry.status = ActiveValue::Set("Finalized".to_string());
        }

        let upd_entry = ver_entry
            .update(&self.db_connection)
            .await
            .map_err(|e| AuthProviderRepoErrors::ErrorUpdatingAuthVerification(e.into()))?;
        let upd_entry2 = auth_entry
            .update(&self.db_connection)
            .await
            .map_err(|e| AuthProviderRepoErrors::ErrorUpdatingAuth(e.into()))?;

        Ok(upd_entry)
    }

    async fn save_token(
        &self,
        id: String,
        base_url: String,
        token: String,
    ) -> anyhow::Result<auth::Model, AuthProviderRepoErrors> {
        let auth = auth::Entity::find_by_id(&id)
            .one(&self.db_connection)
            .await
            .map_err(|e| AuthProviderRepoErrors::ErrorFetchingAuth(e.into()))?
            .ok_or(AuthProviderRepoErrors::AuthNotFound)?;

        let mut auth_entry = auth.into_active_model();
        auth_entry.token = ActiveValue::Set(Some(token));
        auth_entry.status = ActiveValue::Set("Approved".to_string());

        let upd_model = auth_entry
            .update(&self.db_connection)
            .await
            .map_err(|e| AuthProviderRepoErrors::ErrorUpdatingAuth(e.into()))?;

        Ok(upd_model)
    }

    async fn get_auth_by_interact_ref(
        &self,
        interact_ref: String,
    ) -> anyhow::Result<auth_interaction::Model, AuthProviderRepoErrors> {
        let auth = auth_interaction::Entity::find()
            .filter(auth_interaction::Column::InteractRef.eq(interact_ref.clone()))
            .one(&self.db_connection)
            .await
            .map_err(|e| AuthProviderRepoErrors::ErrorFetchingAuthInteraction(e.into()))?
            .ok_or(AuthProviderRepoErrors::AuthInteractionNotFound)?;
        Ok(auth)
    }

    async fn is_token_in_db(&self, token: String) -> anyhow::Result<bool, AuthProviderRepoErrors> {
        let auth = auth::Entity::find()
            .filter(auth::Column::Token.eq(token))
            .one(&self.db_connection)
            .await
            .map_err(|e| AuthProviderRepoErrors::ErrorFetchingAuth(e.into()))?;
        Ok(auth.is_some())
    }

    async fn get_auth_ver_by_id(&self, id: String) -> anyhow::Result<auth_verification::Model, AuthProviderRepoErrors> {
        let auth = auth_verification::Entity::find_by_id(id.clone())
            .one(&self.db_connection)
            .await
            .map_err(|e| AuthProviderRepoErrors::ErrorFetchingAuth(e.into()))?
            .ok_or(AuthProviderRepoErrors::AuthNotFound)?;
        Ok(auth)
    }
}
