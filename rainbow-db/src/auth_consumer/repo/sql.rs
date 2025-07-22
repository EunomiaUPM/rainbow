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

use crate::auth_consumer::entities::auth;
use crate::auth_consumer::entities::auth_interaction;
use crate::auth_consumer::entities::auth_verification;
use crate::auth_consumer::entities::prov;
use crate::auth_consumer::repo::{AuthConsumerRepoErrors, AuthConsumerRepoFactory, AuthConsumerRepoTrait};
use anyhow::anyhow;
use axum::async_trait;
use chrono;
use rainbow_common::auth::gnap::grant_request::Interact4GR;
use sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection, EntityTrait, IntoActiveModel, QuerySelect};
use serde_json::Value;
use url::Url;

#[derive(Clone)]
pub struct AuthConsumerRepoForSql {
    db_connection: DatabaseConnection,
}

impl AuthConsumerRepoForSql {
    fn new(db_connection: DatabaseConnection) -> Self {
        Self { db_connection }
    }
}

impl AuthConsumerRepoFactory for AuthConsumerRepoForSql {
    fn create_repo(db_connection: DatabaseConnection) -> Self
    where
        Self: Sized,
    {
        Self::new(db_connection)
    }
}

#[async_trait]
impl AuthConsumerRepoTrait for AuthConsumerRepoForSql {
    async fn get_all_auths(
        &self,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> anyhow::Result<Vec<auth::Model>, AuthConsumerRepoErrors> {
        let auths = auth::Entity::find()
            .limit(limit.unwrap_or(100000))
            .offset(offset.unwrap_or(0))
            .all(&self.db_connection)
            .await
            .map_err(|e| AuthConsumerRepoErrors::ErrorFetchingAuth(e.into()))?;
        Ok(auths)
    }

    async fn get_auth_by_id(&self, auth_id: String) -> anyhow::Result<auth::Model, AuthConsumerRepoErrors> {
        let auth = auth::Entity::find_by_id(&auth_id)
            .one(&self.db_connection)
            .await
            .map_err(|e| AuthConsumerRepoErrors::ErrorFetchingAuth(e.into()))?
            .ok_or(AuthConsumerRepoErrors::AuthNotFound)?;
        Ok(auth)
    }

    async fn create_auth(
        &self,
        id: String,
        uri: String,
        provider_id: String,
        provider_slug: String,
        actions: String,
        interact: Interact4GR,
    ) -> anyhow::Result<auth::Model, AuthConsumerRepoErrors> {
        let start: Value = Value::String(
            serde_json::to_string(&interact.start).map_err(|e| AuthConsumerRepoErrors::ErrorCreatingAuth(e.into()))?,
        );

        let auth_model = auth::ActiveModel {
            id: ActiveValue::Set(id.clone()),
            assigned_id: ActiveValue::Set(None),
            provider_id: ActiveValue::Set(provider_id),
            provider_slug: ActiveValue::Set(provider_slug),
            actions: ActiveValue::Set(actions),
            status: ActiveValue::Set("Processing".to_string()), // TODO Revisar esto Rodrigo
            token: ActiveValue::Set(None),
            created_at: ActiveValue::Set(chrono::Utc::now().naive_utc()),
            ended_at: ActiveValue::Set(None),
            grant_endpoint: ActiveValue::Set(uri.clone()),
            continue_endpoint: ActiveValue::Set(None),
        };

        let auth_interaction_model = auth_interaction::ActiveModel {
            id: ActiveValue::Set(id.clone()),
            start: ActiveValue::Set(start),
            method: ActiveValue::Set(interact.finish.method),
            uri: ActiveValue::Set(interact.finish.uri),
            client_nonce: ActiveValue::Set(interact.finish.nonce),
            as_nonce: ActiveValue::Set(None),
            interact_ref: ActiveValue::Set(None),
            grant_endpoint: ActiveValue::Set(uri),
            hash: ActiveValue::Set(None),
            hash_method: ActiveValue::Set(interact.finish.hash_method),
            hints: ActiveValue::Set(None), // TODO ??
        };

        let auth = auth::Entity::insert(auth_model)
            .exec_with_returning(&self.db_connection)
            .await
            .map_err(|e| AuthConsumerRepoErrors::ErrorCreatingAuth(e.into()))?;

        let auth_interaction = auth_interaction::Entity::insert(auth_interaction_model)
            .exec_with_returning(&self.db_connection)
            .await
            .map_err(|e| AuthConsumerRepoErrors::ErrorCreatingAuthInteraction(e.into()))?;
        Ok(auth)
    }

    async fn auth_pending(
        &self,
        id: String,
        assigned_id: String,
        continue_uri: String,
        as_nonce: String,
    ) -> anyhow::Result<auth::Model, AuthConsumerRepoErrors> {
        let mut entry = auth::Entity::find_by_id(&id)
            .one(&self.db_connection)
            .await
            .map_err(|e| AuthConsumerRepoErrors::ErrorCreatingAuth(e.into()))?
            .ok_or(AuthConsumerRepoErrors::AuthNotFound)?
            .into_active_model();

        let mut entry_int = auth_interaction::Entity::find_by_id(&id)
            .one(&self.db_connection)
            .await
            .map_err(|e| AuthConsumerRepoErrors::ErrorCreatingAuthInteraction(e.into()))?
            .ok_or(AuthConsumerRepoErrors::AuthNotFound)?
            .into_active_model();

        entry_int.as_nonce = ActiveValue::Set(Some(as_nonce));
        entry.status = ActiveValue::Set("Pending".to_string());
        entry.assigned_id = ActiveValue::Set(Some(assigned_id));
        entry.continue_endpoint = ActiveValue::Set(Some(continue_uri));

        let upd_entry =
            entry.update(&self.db_connection).await.map_err(|e| AuthConsumerRepoErrors::ErrorUpdatingAuth(e.into()))?;

        let _ = entry_int
            .update(&self.db_connection)
            .await
            .map_err(|e| AuthConsumerRepoErrors::ErrorUpdatingAuthInteraction(e.into()))?;

        Ok(upd_entry)
    }

    async fn delete_auth(&self, id: String) -> anyhow::Result<auth::Model, AuthConsumerRepoErrors> {
        let mut entry = auth::Entity::find_by_id(&id)
            .one(&self.db_connection)
            .await
            .map_err(|e| AuthConsumerRepoErrors::ErrorCreatingAuth(e.into()))?
            .ok_or(AuthConsumerRepoErrors::AuthNotFound)?;

        let ret = entry.clone();
        let active_model = entry.into_active_model();
        active_model
            .delete(&self.db_connection)
            .await
            .map_err(|e| AuthConsumerRepoErrors::ErrorDeletingAuth(e.into()))?;

        Ok(ret)
    }

    async fn get_interaction_by_id(
        &self,
        id: String,
    ) -> anyhow::Result<auth_interaction::Model, AuthConsumerRepoErrors> {
        let auth_interaction = auth_interaction::Entity::find_by_id(&id)
            .one(&self.db_connection)
            .await
            .map_err(|e| AuthConsumerRepoErrors::ErrorFetchingAuthInteraction(e.into()))?
            .ok_or(AuthConsumerRepoErrors::AuthInteractionNotFound)?;

        Ok(auth_interaction)
    }

    async fn update_interaction_by_id(
        &self,
        id: String,
        interact_ref: String,
        hash: String,
    ) -> anyhow::Result<auth_interaction::Model, AuthConsumerRepoErrors> {
        let mut entry = auth_interaction::Entity::find_by_id(&id)
            .one(&self.db_connection)
            .await
            .map_err(|e| AuthConsumerRepoErrors::ErrorFetchingAuthInteraction(e.into()))?
            .ok_or(AuthConsumerRepoErrors::AuthInteractionNotFound)?
            .into_active_model();

        entry.interact_ref = ActiveValue::Set(Some(interact_ref));
        entry.hash = ActiveValue::Set(Some(hash));
        let upd_entry = entry
            .update(&self.db_connection)
            .await
            .map_err(|e| AuthConsumerRepoErrors::ErrorUpdatingAuthInteraction(e.into()))?;
        Ok(upd_entry)
    }

    async fn create_auth_verification(
        &self,
        id: String,
        uri: String,
    ) -> anyhow::Result<auth_verification::Model, AuthConsumerRepoErrors> {
        if !uri.contains("openid4vp") {
            return Err(AuthConsumerRepoErrors::ErrorCreatingAuthVerification(
                anyhow!("Invalid format for uri, not contains openid4vp"),
            ));
        }
        let fixed_uri = uri.replacen("openid4vp://", "https://", 1);
        let url = Url::parse(&fixed_uri).map_err(|e| {
            AuthConsumerRepoErrors::ErrorCreatingAuthVerification(anyhow!("Invalid URI: {}", fixed_uri))
        })?;

        let response_type = url.query_pairs().find(|(k, _)| k == "response_type").map(|(_, v)| v.into_owned());
        let client_id = url.query_pairs().find(|(k, _)| k == "client_id").map(|(_, v)| v.into_owned());
        let response_mode = url.query_pairs().find(|(k, _)| k == "response_mode").map(|(_, v)| v.into_owned());
        let pd_uri = url.query_pairs().find(|(k, _)| k == "presentation_definition_uri").map(|(_, v)| v.into_owned());
        let client_id_scheme = url.query_pairs().find(|(k, _)| k == "client_id_scheme").map(|(_, v)| v.into_owned());
        let nonce = url.query_pairs().find(|(k, _)| k == "nonce").map(|(_, v)| v.into_owned());
        let response_uri = url.query_pairs().find(|(k, _)| k == "response_uri").map(|(_, v)| v.into_owned());

        let auth_verification_model = auth_verification::ActiveModel {
            id: ActiveValue::Set(id),
            scheme: ActiveValue::Set("openid4vp".to_string()),
            response_type: ActiveValue::Set(response_type.unwrap()),
            client_id: ActiveValue::Set(client_id.unwrap()),
            response_mode: ActiveValue::Set(response_mode.unwrap()),
            pd_uri: ActiveValue::Set(pd_uri.unwrap()),
            client_id_scheme: ActiveValue::Set(client_id_scheme.unwrap()),
            nonce: ActiveValue::Set(nonce.unwrap()),
            response_uri: ActiveValue::Set(response_uri.unwrap()),
            uri: ActiveValue::Set(uri),
            status: ActiveValue::Set("Pending".to_string()),
            created_at: ActiveValue::Set(chrono::Utc::now().naive_utc()),
            ended_at: ActiveValue::Set(None),
        };

        let auth_verification_model = auth_verification::Entity::insert(auth_verification_model)
            .exec_with_returning(&self.db_connection)
            .await
            .map_err(|e| AuthConsumerRepoErrors::ErrorCreatingAuthVerification(e.into()))?;
        Ok(auth_verification_model)
    }

    async fn grant_req_approved(&self, id: String, jwt: String) -> anyhow::Result<auth::Model, AuthConsumerRepoErrors> {
        let mut entry = auth::Entity::find_by_id(&id)
            .one(&self.db_connection)
            .await
            .map_err(|e| AuthConsumerRepoErrors::ErrorFetchingAuth(e.into()))?
            .ok_or(AuthConsumerRepoErrors::AuthNotFound)?
            .into_active_model();
        entry.status = ActiveValue::Set("Approved".to_string());
        entry.token = ActiveValue::Set(Some(jwt));
        let upd_entry =
            entry.update(&self.db_connection).await.map_err(|e| AuthConsumerRepoErrors::ErrorUpdatingAuth(e.into()))?;
        Ok(upd_entry)
    }

    async fn create_prov(
        &self,
        provider: String,
        provider_route: String,
    ) -> anyhow::Result<(), AuthConsumerRepoErrors> {
        let prov_model = prov::ActiveModel {
            provider: ActiveValue::Set(provider.clone()),
            provider_route: ActiveValue::Set(provider_route),
            onboard: ActiveValue::Set(false),
        };

        let prov = prov::Entity::insert(prov_model).exec_with_returning(&self.db_connection).await;

        match prov {
            Ok(_) => Ok(()),
            Err(e) if e.to_string().contains("duplicate key") => Ok(()),
            Err(e) => Err(AuthConsumerRepoErrors::ErrorCreatingAuthProvider(e.into())),
        }
    }

    async fn prov_onboard(&self, provider: String) -> anyhow::Result<(), AuthConsumerRepoErrors> {
        let mut entry = prov::Entity::find_by_id(&provider)
            .one(&self.db_connection)
            .await
            .map_err(|e| AuthConsumerRepoErrors::ErrorFetchingAuthProvider(e.into()))?
            .ok_or(AuthConsumerRepoErrors::AuthProviderNotFound)?
            .into_active_model();

        entry.onboard = ActiveValue::Set(true);
        let upd_entry = entry
            .update(&self.db_connection)
            .await
            .map_err(|e| AuthConsumerRepoErrors::ErrorUpdatingAuthProvider(e.into()))?;

        Ok(())
    }

    async fn get_all_provs(&self) -> anyhow::Result<Vec<prov::Model>, AuthConsumerRepoErrors> {
        let provs = prov::Entity::find()
            .limit(100000)
            .offset(0)
            .all(&self.db_connection)
            .await
            .map_err(|e| AuthConsumerRepoErrors::ErrorFetchingAuthProvider(e.into()))?;
        Ok(provs)
    }

    async fn get_prov(&self, provider: String) -> anyhow::Result<prov::Model, AuthConsumerRepoErrors> {
        let prov = prov::Entity::find_by_id(&provider)
            .one(&self.db_connection)
            .await
            .map_err(|e| AuthConsumerRepoErrors::ErrorFetchingAuthProvider(e.into()))?
            .ok_or(AuthConsumerRepoErrors::AuthProviderNotFound)?;
        Ok(prov)
    }
}
