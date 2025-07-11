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
use crate::auth_provider::repo::{AuthProviderRepoFactory, AuthProviderRepoTrait};
use anyhow::bail;
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
    async fn get_all_auths(&self, limit: Option<u64>, offset: Option<u64>) -> anyhow::Result<Vec<auth::Model>> {
        let auths = auth::Entity::find()
            .limit(limit.unwrap_or(100000))
            .offset(offset.unwrap_or(0))
            .all(&self.db_connection)
            .await;
        match auths {
            Ok(auths) => Ok(auths),
            Err(e) => bail!("Failed to fetch data: {}", e),
        }
    }

    async fn get_auth_by_id(&self, auth_id: String) -> anyhow::Result<auth::Model> {
        let auth = auth::Entity::find_by_id(auth_id.clone()).one(&self.db_connection).await;

        match auth {
            Ok(Some(auth)) => Ok(auth),
            Ok(None) => bail!("NO authentication with id {}", auth_id),
            Err(e) => bail!("Failed to fetch data: {}", e),
        }
    }

    async fn create_auth(
        &self,
        consumer: String,
        audience: String,
        grant_uri: String,
        actions: String,
        interact: Interact4GR,
    ) -> anyhow::Result<(
        auth::Model,
        auth_interaction::Model,
        auth_verification::Model,
    )> {
        let id = uuid::Uuid::new_v4().to_string();

        let state: String = rand::thread_rng().sample_iter(&Alphanumeric).take(12).map(char::from).collect();
        let as_nonce: String = rand::thread_rng().sample_iter(&Alphanumeric).take(36).map(char::from).collect();
        let nonce: String = rand::thread_rng().sample_iter(&Alphanumeric).take(12).map(char::from).collect();
        let interact_ref: String = rand::thread_rng().sample_iter(&Alphanumeric).take(16).map(char::from).collect();

        let start: Value = Value::String(serde_json::to_string(&interact.start)?);

        let hash_method = interact.finish.hash_method.unwrap_or_else(|| "sha-256".to_string());
        let hash_input = format!(
            "{}\n{}\n{}\n{}",
            interact.finish.nonce, as_nonce, interact_ref, grant_uri
        );

        let mut hasher = Sha256::new();
        hasher.update(hash_input.as_bytes());
        let result = hasher.finalize();

        let hash = URL_SAFE_NO_PAD.encode(result);

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

        let auth = auth::Entity::insert(auth_model).exec_with_returning(&self.db_connection).await?;
        let auth_interaction =
            auth_interaction::Entity::insert(auth_interaction_model).exec_with_returning(&self.db_connection).await?;
        let auth_verification =
            auth_verification::Entity::insert(auth_verification_model).exec_with_returning(&self.db_connection).await?;

        Ok((auth, auth_interaction, auth_verification))
    }

    async fn create_truncated_auth(&self, audience: String, state: String) -> anyhow::Result<(auth::Model, auth_verification::Model)> {
        let id = uuid::Uuid::new_v4().to_string();

        let as_nonce: String = rand::thread_rng().sample_iter(&Alphanumeric).take(36).map(char::from).collect();
        let nonce: String = rand::thread_rng().sample_iter(&Alphanumeric).take(12).map(char::from).collect();
        let interact_ref: String = rand::thread_rng().sample_iter(&Alphanumeric).take(16).map(char::from).collect();

        // let start: Value = Value::String(serde_json::to_string(&interact.start)?);

        // let hash_method = interact.finish.hash_method.unwrap_or_else(|| "sha-256".to_string());
        // let hash_input = format!(
        //     "{}\n{}\n{}\n{}",
        //     interact.finish.nonce, as_nonce, interact_ref, grant_uri
        // );

        // let mut hasher = Sha256::new();
        // hasher.update(hash_input.as_bytes());
        // let result = hasher.finalize();

        // let hash = URL_SAFE_NO_PAD.encode(result);

        let auth_model = auth::ActiveModel {
            id: ActiveValue::Set(id.clone()),
            consumer: ActiveValue::Set(None),
            actions: ActiveValue::Set("talk".to_string()),
            status: ActiveValue::Set("Pending".to_string()),
            token: ActiveValue::Set(None),
            created_at: ActiveValue::Set(chrono::Utc::now().naive_utc()),
            ended_at: ActiveValue::Set(None),
        };

        // let auth_interaction_model = auth_interaction::ActiveModel {
        //     id: ActiveValue::Set(id.clone()),
        //     start: ActiveValue::Set(start),
        //     method: ActiveValue::Set(interact.finish.method),
        //     uri: ActiveValue::Set(interact.finish.uri),
        //     client_nonce: ActiveValue::Set(interact.finish.nonce),
        //     as_nonce: ActiveValue::Set(as_nonce),
        //     interact_ref: ActiveValue::Set(interact_ref),
        //     grant_endpoint: ActiveValue::Set(grant_uri),
        //     hash: ActiveValue::Set(hash),
        //     hash_method: ActiveValue::Set(Some(hash_method)),
        //     hints: ActiveValue::Set(None), // TODO
        // };

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

        let auth = auth::Entity::insert(auth_model).exec_with_returning(&self.db_connection).await?;
        // let auth_interaction =
        //     auth_interaction::Entity::insert(auth_interaction_model).exec_with_returning(&self.db_connection).await?;
        let auth_verification =
            auth_verification::Entity::insert(auth_verification_model).exec_with_returning(&self.db_connection).await?;

        Ok((auth, auth_verification))
    }

    async fn update_auth_status(&self, id: String, status: String, end: bool) -> anyhow::Result<auth::Model> {
        let mut entry = match auth::Entity::find_by_id(id.clone()).one(&self.db_connection).await {
            Ok(Some(entry)) => entry.into_active_model(),
            Ok(None) => bail!("No entry auth with ID: {}", id),
            Err(e) => bail!("Failed to fetch data: {}", e),
        };

        entry.status = ActiveValue::Set(status);
        if end {
            entry.ended_at = ActiveValue::Set(Some(chrono::Utc::now().naive_utc()));
        }

        let upd_entry = entry.update(&self.db_connection).await;

        match upd_entry {
            Ok(upd_entry) => Ok(upd_entry),
            Err(e) => bail!("Failed to update status: {}", e),
        }
    }

    async fn delete_auth(&self, id: String) -> anyhow::Result<auth::Model> {
        let mut entry = match auth::Entity::find_by_id(id.clone()).one(&self.db_connection).await {
            Ok(Some(entry)) => entry,
            Ok(None) => bail!("No entry found with ID: {}", id),
            Err(e) => bail!("Failed to fetch data: {}", e),
        };
        let ret = entry.clone();
        let active_model = entry.into_active_model();
        active_model.delete(&self.db_connection).await?;

        Ok(ret)
    }

    async fn get_interaction_by_id(&self, id: String) -> anyhow::Result<auth_interaction::Model> {
        let auth_interaction = auth_interaction::Entity::find_by_id(id.clone()).one(&self.db_connection).await;

        match auth_interaction {
            Ok(Some(auth_interaction)) => Ok(auth_interaction),
            Ok(None) => bail!("No Interaction from authentication with id {}", id),
            Err(e) => bail!("Failed to fetch data: {}", e),
        }
    }

    async fn get_auth_by_state(&self, state: String) -> anyhow::Result<auth_verification::Model> {
        let auth = auth_verification::Entity::find()
            .filter(auth_verification::Column::State.eq(state.clone()))
            .one(&self.db_connection)
            .await;

        match auth {
            Ok(Some(auth)) => Ok(auth),
            Ok(None) => bail!("No verification from authentication with state {}", state),
            Err(e) => bail!("Failed to fetch data: {}", e),
        }
    }

    async fn get_av_by_id_update_holder(
        &self,
        id: String,
        vpt: String,
        holder: String,
    ) -> anyhow::Result<auth_verification::Model> {
        let auth_ver = auth_verification::Entity::find_by_id(&id).one(&self.db_connection).await;

        let mut entry = match auth_ver {
            Ok(Some(auth_ver)) => auth_ver.into_active_model(),
            Ok(None) => bail!("No Verification from authentication with id {}", id),
            Err(e) => bail!("Failed to fetch data: {}", e),
        };

        entry.holder = ActiveValue::Set(Some(holder));
        entry.vpt = ActiveValue::Set(Some(vpt));

        let upd_entry = entry.update(&self.db_connection).await;

        match upd_entry {
            Ok(upd_entry) => Ok(upd_entry),
            Err(e) => bail!("Failed to update status: {}", e),
        }
    }

    async fn update_verification_result(&self, id: String, result: bool) -> anyhow::Result<auth_verification::Model> {
        let auth_ver = auth_verification::Entity::find_by_id(&id).one(&self.db_connection).await;
        let auth = auth::Entity::find_by_id(&id).one(&self.db_connection).await;

        let mut ver_entry = match auth_ver {
            Ok(Some(auth_ver)) => auth_ver.into_active_model(),
            Ok(None) => bail!("No Verification from authentication with id {}", id),
            Err(e) => bail!("Failed to fetch data: {}", e),
        };
        let mut auth_entry = match auth {
            Ok(Some(auth)) => auth.into_active_model(),
            Ok(None) => bail!("No authentication with id {}", id),
            Err(e) => bail!("Failed to fetch data: {}", e),
        };

        ver_entry.success = ActiveValue::Set(Some(result));
        ver_entry.ended_at = ActiveValue::Set(Some(chrono::Utc::now().naive_utc()));
        if result {
            auth_entry.status = ActiveValue::Set("Processing".to_string());
        } else {
            auth_entry.status = ActiveValue::Set("Finalized".to_string());
        }

        let upd_entry = ver_entry.update(&self.db_connection).await;
        let upd_entry2 = auth_entry.update(&self.db_connection).await;

        match upd_entry2 {
            Ok(auth_entry) => (),
            Err(e) => bail!("Failed to update status: {}", e),
        }
        match upd_entry {
            Ok(upd_entry) => {Ok(upd_entry)}
            Err(e) => bail!("Failed to update status: {}", e),
        }
    }

    async fn save_token(&self, id: String, base_url: String, token: String) -> anyhow::Result<auth::Model> {
        let auth = auth::Entity::find_by_id(&id).one(&self.db_connection).await;

        let mut auth_entry = match auth {
            Ok(Some(auth)) => auth.into_active_model(),
            Ok(None) => bail!("No authentication with id {}", id),
            Err(e) => bail!("Failed to fetch data: {}", e),
        };

        auth_entry.token = ActiveValue::Set(Some(token));
        auth_entry.status = ActiveValue::Set("Approved".to_string());

        let upd_model = auth_entry.update(&self.db_connection).await;

        match upd_model {
            Ok(upd_entry) => Ok(upd_entry),
            Err(e) => bail!("Failed to save in db: {}", e),
        }
    }

    async fn get_auth_by_interact_ref(&self, interact_ref: String) -> anyhow::Result<auth_interaction::Model> {
        let auth = auth_interaction::Entity::find()
            .filter(auth_interaction::Column::InteractRef.eq(interact_ref.clone()))
            .one(&self.db_connection)
            .await;

        match auth {
            Ok(Some(auth)) => Ok(auth),
            Ok(None) => bail!(
                "No verification from authentication with interact_ref {}",
                interact_ref
            ),
            Err(e) => bail!("Failed to fetch data: {}", e),
        }
    }

    async fn is_token_in_db(&self, token: String) -> anyhow::Result<bool> {
        let auth = auth::Entity::find().filter(auth::Column::Token.eq(token)).one(&self.db_connection).await;

        match auth {
            Ok(Some(auth)) => Ok(true),
            Ok(None) => Ok(false),
            Err(e) => bail!("Failed to fetch data: {}", e),
        }
    }

    async fn get_auth_ver_by_id(&self, id: String) -> anyhow::Result<auth_verification::Model> {
        let auth = auth_verification::Entity::find_by_id(id.clone()).one(&self.db_connection).await;

        match auth {
            Ok(Some(auth)) => Ok(auth),
            Ok(None) => bail!("NO authentication with id {}", id),
            Err(e) => bail!("Failed to fetch data: {}", e),
        }
    }
}
