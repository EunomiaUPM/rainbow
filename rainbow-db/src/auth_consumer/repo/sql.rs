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

use crate::auth_consumer::entities::auth;
use crate::auth_consumer::entities::auth::Status;
use crate::auth_consumer::entities::auth_interaction;
use crate::auth_consumer::entities::auth_verification;
use crate::auth_consumer::repo::AuthConsumerRepoTrait;
use anyhow::{anyhow, bail};
use axum::async_trait;
use chrono;
use rainbow_common::auth::Interact4GR;
use rainbow_common::config::database::get_db_connection;
use rand::{distributions::Alphanumeric, Rng};
use sea_orm::sqlx::types::uuid;
use sea_orm::{ActiveModelTrait, ActiveValue, EntityTrait, IntoActiveModel, QuerySelect};
use serde_json::Value;
use url::Url;

pub struct AuthConsumerRepo {}

#[async_trait]
impl AuthConsumerRepoTrait for AuthConsumerRepo {
    async fn get_all_auths(
        &self,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> anyhow::Result<Vec<auth::Model>> {
        let db_connection = get_db_connection().await;
        let auths = auth::Entity::find()
            .limit(limit.unwrap_or(100000))
            .offset(offset.unwrap_or(0))
            .all(db_connection)
            .await;
        match auths {
            Ok(auths) => Ok(auths),
            Err(e) => bail!("Failed to fetch data: {}", e),
        }
    }

    async fn get_auth_by_id(&self, auth_id: i64) -> anyhow::Result<auth::Model> {
        let db_connection = get_db_connection().await;
        let auth = auth::Entity::find_by_id(auth_id).one(db_connection).await;

        match auth {
            Ok(Some(auth)) => Ok(auth),
            Ok(None) => bail!("NO authentication with id {}", auth_id),
            Err(e) => bail!("Failed to fetch data: {}", e),
        }
    }

    async fn create_auth(
        &self,
        provider: String,
        actions: Vec<String>,
        interact: Interact4GR,
    ) -> anyhow::Result<auth::Model> {
        let db_connection = get_db_connection().await;

        let actions: Value = Value::String(serde_json::to_string(&actions).unwrap());
        let start: Value = Value::String(serde_json::to_string(&interact.start).unwrap());

        let auth_model = auth::ActiveModel {
            assigned_id: ActiveValue::Set(None),
            provider: ActiveValue::Set(provider),
            actions: ActiveValue::Set(actions),
            status: ActiveValue::Set(Status::Requested),
            created_at: ActiveValue::Set(chrono::Utc::now().naive_utc()),
            ended_at: ActiveValue::Set(None),
            ..Default::default()
        };

        let auth_interaction_model = auth_interaction::ActiveModel {
            start: ActiveValue::Set(start),
            method: ActiveValue::Set(interact.finish.method),
            uri: ActiveValue::Set(interact.finish.uri),
            nonce: ActiveValue::Set(interact.finish.nonce),
            hash_method: ActiveValue::Set(interact.finish.hash_method),
            hints: ActiveValue::Set(None), // FUERA PROBLEMAS
            ..Default::default()
        };

        let auth = auth::Entity::insert(auth_model).exec_with_returning(db_connection).await?;
        let auth_interaction = auth_interaction::Entity::insert(auth_interaction_model)
            .exec_with_returning(db_connection)
            .await?;

        Ok(auth)
    }

    async fn auth_accepted(&self, id: i64, assigned_id: String) -> anyhow::Result<auth::Model> {
        let db_connection = get_db_connection().await;

        let mut entry = match auth::Entity::find_by_id(id).one(db_connection).await {
            Ok(Some(entry)) => entry.into_active_model(),
            Ok(None) => bail!("No entry auth with ID: {}", id),
            Err(e) => bail!("Failed to fetch data: {}", e),
        };

        entry.status = ActiveValue::Set(Status::Ongoing);
        entry.assigned_id = ActiveValue::Set(Some(assigned_id));

        let upd_entry = entry.update(db_connection).await?;

        Ok(upd_entry)
    }

    async fn delete_auth(&self, id: i64) -> anyhow::Result<auth::Model> {
        let db_connection = get_db_connection().await;

        let mut entry = match auth::Entity::find_by_id(id).one(db_connection).await {
            Ok(Some(entry)) => entry,
            Ok(None) => bail!("No entry found with ID: {}", id),
            Err(e) => bail!("Failed to fetch data: {}", e),
        };
        let ret = entry.clone();
        let active_model = entry.into_active_model();
        active_model.delete(db_connection).await?;

        Ok(ret)
    }

    async fn get_interaction_by_id(&self, id: i64) -> anyhow::Result<auth_interaction::Model> {
        let db_connection = get_db_connection().await;

        let auth_interaction = auth_interaction::Entity::find_by_id(id).one(db_connection).await;

        match auth_interaction {
            Ok(Some(auth_interaction)) => Ok(auth_interaction),
            Ok(None) => bail!("No Inteaction from authentication with id {}", id),
            Err(e) => bail!("Failed to fetch data: {}", e),
        }
    }

    async fn create_auth_verification(
        &self,
        id: i64,
        uri: String,
    ) -> anyhow::Result<auth_verification::Model> {
        let db_connection = get_db_connection().await;

        if !uri.contains("openid4vp") {
            bail!("Invalid format for uri")
        }
        let fixed_uri = uri.replacen("openid4vp://", "https://", 1);
        let url = Url::parse(&fixed_uri).map_err(|_| anyhow!("Invalid URI: {}", fixed_uri))?;

        let response_type =
            url.query_pairs().find(|(k, _)| k == "response_type").map(|(_, v)| v.into_owned());
        let client_id =
            url.query_pairs().find(|(k, _)| k == "client_id").map(|(_, v)| v.into_owned());
        let response_mode =
            url.query_pairs().find(|(k, _)| k == "response_mode").map(|(_, v)| v.into_owned());
        let pd_uri = url
            .query_pairs()
            .find(|(k, _)| k == "presentation_definition_uri")
            .map(|(_, v)| v.into_owned());
        let client_id_scheme =
            url.query_pairs().find(|(k, _)| k == "client_id_scheme").map(|(_, v)| v.into_owned());
        let nonce = url.query_pairs().find(|(k, _)| k == "nonce").map(|(_, v)| v.into_owned());
        let response_uri =
            url.query_pairs().find(|(k, _)| k == "response_uri").map(|(_, v)| v.into_owned());

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
        };

        let auth_verification_model = auth_verification::Entity::insert(auth_verification_model)
            .exec_with_returning(db_connection)
            .await?;
        Ok(auth_verification_model)
    }
}
