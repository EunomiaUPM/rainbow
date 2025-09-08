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
use std::any::TypeId;
use crate::data::entities::{auth, auth_interaction, auth_verification};
use crate::data::repo::{AuthInteractionRepoTrait, AuthRepoTrait, AuthVerificationRepoTrait, AuthorityRepoFactory, BasicRepoTrait};
use anyhow::bail;
use sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection, EntityTrait, IntoActiveModel, QuerySelect};

#[derive(Clone)]
pub struct AuthorityRepoForSql {
    db_connection: DatabaseConnection,
}

impl AuthorityRepoForSql {
    fn new(db_connection: DatabaseConnection) -> Self {
        Self { db_connection }
    }
}

impl AuthorityRepoFactory for AuthorityRepoForSql {
    fn create_repo<T>(db_connection: DatabaseConnection) -> Box<dyn BasicRepoTrait<T>>
    where
        T: Send + Sync + 'static
    {
        if TypeId::of::<T>() == TypeId::of::<auth::Model>() {
            Box::new(AuthRepo::new(db_connection)) as Box<dyn BasicRepoTrait<T>>
        } else if TypeId::of::<T>() == TypeId::of::<auth_interaction::Model>() {
            Box::new(AuthInteractionRepo::new(db_connection)) as Box<dyn BasicRepoTrait<T>>
        } else if TypeId::of::<T>() == TypeId::of::<auth_verification::Model>() {
            Box::new(AuthVerificationRepo::new(db_connection)) as Box<dyn BasicRepoTrait<T>>
        } else {
            panic!("No repo implementation for this type");
        }
    }
}



//
// impl AuthRepoTrait for AuthorityRepoForSql {
//     async fn get_all_auths(&self, limit: Option<u64>, offset: Option<u64>) -> anyhow::Result<Vec<auth::Model>> {
//         let models = auth::Entity::find()
//             .limit(limit.unwrap_or(100000))
//             .offset(offset.unwrap_or(0))
//             .all(&self.db_connection)
//             .await?;
//         Ok(models)
//     }
//
//     async fn get_auth_by_id(&self, id: &str) -> anyhow::Result<Option<auth::Model>> {
//         let model = auth::Entity::find_by_id(id).one(&self.db_connection).await?;
//         Ok(model)
//     }
//
//     async fn create_auth(&self, model: auth::Model) -> anyhow::Result<auth::Model> {
//         let model = auth::ActiveModel {
//             id: ActiveValue::Set(model.id),
//             client: ActiveValue::Set(model.client),
//             actions: ActiveValue::Set(model.actions),
//             status: ActiveValue::Set(model.status),
//             token: ActiveValue::Set(model.token),
//             created_at: ActiveValue::Set(model.created_at),
//             ended_at: ActiveValue::Set(model.ended_at),
//         };
//
//         let new_model = auth::Entity::insert(model).exec_with_returning(&self.db_connection).await?;
//         Ok(new_model)
//     }
//
//     async fn delete_auth(&self, id: &str) -> anyhow::Result<()> {
//         let mut active_model = match auth::Entity::find_by_id(id).one(&self.db_connection).await? {
//             Some(model) => model.into_active_model(),
//             None => bail!("No entry found with ID: {}", id),
//         };
//         active_model.delete(&self.db_connection).await?;
//
//         Ok(())
//     }
//
//     async fn edit_auth(&self, model: auth::Model) -> anyhow::Result<auth::Model> {
//         let mut active_model = match auth::Entity::find_by_id(&model.id).one(&self.db_connection).await? {
//             Some(auth) => auth.into_active_model(),
//             None => bail!("No auth found with ID: {}", &model.id),
//         };
//
//         active_model.client = ActiveValue::Set(model.client);
//         active_model.actions = ActiveValue::Set(model.actions);
//         active_model.status = ActiveValue::Set(model.status);
//         active_model.token = ActiveValue::Set(model.token);
//         active_model.ended_at = ActiveValue::Set(model.ended_at);
//
//         let new_model = active_model.update(&self.db_connection).await?;
//         Ok(new_model)
//     }
// }
//
// impl AuthInteractionRepoTrait for AuthorityRepoForSql {
//     async fn get_all_auths_int(
//         &self,
//         limit: Option<u64>,
//         offset: Option<u64>,
//     ) -> anyhow::Result<Vec<auth_interaction::Model>> {
//         let models = auth_interaction::Entity::find()
//             .limit(limit.unwrap_or(100000))
//             .offset(offset.unwrap_or(0))
//             .all(&self.db_connection)
//             .await?;
//         Ok(models)
//     }
//
//     async fn get_auth_int_by_id(&self, id: &str) -> anyhow::Result<Option<auth_interaction::Model>> {
//         let model = auth_interaction::Entity::find_by_id(id).one(&self.db_connection).await?;
//         Ok(model)
//     }
//
//     async fn create_auth_int(&self, model: auth_interaction::Model) -> anyhow::Result<auth_interaction::Model> {
//         let model = auth_interaction::ActiveModel {
//             id: ActiveValue::Set(model.id),
//             start: ActiveValue::Set(model.start),
//             method: ActiveValue::Set(model.method),
//             uri: ActiveValue::Set(model.uri),
//             client_nonce: ActiveValue::Set(model.client_nonce),
//             as_nonce: ActiveValue::Set(model.as_nonce),
//             interact_ref: ActiveValue::Set(model.interact_ref),
//             grant_endpoint: ActiveValue::Set(model.grant_endpoint),
//             hash: ActiveValue::Set(model.hash),
//             hash_method: ActiveValue::Set(model.hash_method),
//             hints: ActiveValue::Set(model.hints),
//         };
//
//         let new_model = auth_interaction::Entity::insert(model).exec_with_returning(&self.db_connection).await?;
//         Ok(new_model)
//     }
//
//     async fn delete_auth_int(&self, id: &str) -> anyhow::Result<()> {
//         let mut active_model = match auth_interaction::Entity::find_by_id(id).one(&self.db_connection).await? {
//             Some(model) => model.into_active_model(),
//             None => bail!("No entry found with ID: {}", id),
//         };
//         active_model.delete(&self.db_connection).await?;
//
//         Ok(())
//     }
//
//     async fn edit_auth_int(&self, model: auth_interaction::Model) -> anyhow::Result<auth_interaction::Model> {
//         let mut active_model = match auth_interaction::Entity::find_by_id(&model.id).one(&self.db_connection).await? {
//             Some(model) => model.into_active_model(),
//             None => bail!("No auth found with ID: {}", &model.id),
//         };
//
//         active_model.start = ActiveValue::Set(model.start);
//         active_model.method = ActiveValue::Set(model.method);
//         active_model.uri = ActiveValue::Set(model.uri);
//         active_model.client_nonce = ActiveValue::Set(model.client_nonce);
//         active_model.as_nonce = ActiveValue::Set(model.as_nonce);
//         active_model.interact_ref = ActiveValue::Set(model.interact_ref);
//         active_model.grant_endpoint = ActiveValue::Set(model.grant_endpoint);
//         active_model.hash = ActiveValue::Set(model.hash);
//         active_model.hash_method = ActiveValue::Set(model.hash_method);
//         active_model.hints = ActiveValue::Set(model.hints);
//
//         let new_model = active_model.update(&self.db_connection).await?;
//         Ok(new_model)
//     }
// }
//
// impl AuthVerificationRepoTrait for AuthorityRepoForSql {
//     async fn get_all_auths_ver(
//         &self,
//         limit: Option<u64>,
//         offset: Option<u64>,
//     ) -> anyhow::Result<Vec<auth_verification::Model>> {
//         let models = auth_verification::Entity::find()
//             .limit(limit.unwrap_or(100000))
//             .offset(offset.unwrap_or(0))
//             .all(&self.db_connection)
//             .await?;
//         Ok(models)
//     }
//
//     async fn get_auth_ver_by_id(&self, id: &str) -> anyhow::Result<Option<auth_verification::Model>> {
//         let model = auth_verification::Entity::find_by_id(id).one(&self.db_connection).await?;
//         Ok(model)
//     }
//
//     async fn create_auth_ver(&self, model: auth_verification::Model) -> anyhow::Result<auth_verification::Model> {
//         let model = auth_verification::ActiveModel {
//             id: ActiveValue::Set(model.id),
//             state: ActiveValue::Set(model.state),
//             nonce: ActiveValue::Set(model.nonce),
//             audience: ActiveValue::Set(model.audience),
//             holder: ActiveValue::Set(model.holder),
//             vpt: ActiveValue::Set(model.vpt),
//             success: ActiveValue::Set(model.success),
//             status: ActiveValue::Set(model.status),
//             created_at: ActiveValue::Set(model.created_at),
//             ended_at: ActiveValue::Set(model.ended_at),
//         };
//
//         let new_model = auth_verification::Entity::insert(model).exec_with_returning(&self.db_connection).await?;
//         Ok(new_model)
//     }
//
//     async fn delete_auth_ver(&self, id: &str) -> anyhow::Result<()> {
//         let mut active_model = match auth_verification::Entity::find_by_id(id).one(&self.db_connection).await? {
//             Some(model) => model.into_active_model(),
//             None => bail!("No entry found with ID: {}", id),
//         };
//         active_model.delete(&self.db_connection).await?;
//
//         Ok(())
//     }
//
//     async fn edit_auth_ver(&self, model: auth_verification::Model) -> anyhow::Result<auth_verification::Model> {
//         let mut active_model = match auth_verification::Entity::find_by_id(&model.id).one(&self.db_connection).await? {
//             Some(model) => model.into_active_model(),
//             None => bail!("No auth found with ID: {}", &model.id),
//         };
//
//         active_model.state = ActiveValue::Set(model.state);
//         active_model.nonce = ActiveValue::Set(model.nonce);
//         active_model.audience = ActiveValue::Set(model.audience);
//         active_model.holder = ActiveValue::Set(model.holder);
//         active_model.vpt = ActiveValue::Set(model.vpt);
//         active_model.success = ActiveValue::Set(model.success);
//         active_model.status = ActiveValue::Set(model.status);
//         active_model.created_at = ActiveValue::Set(model.created_at);
//         active_model.ended_at = ActiveValue::Set(model.ended_at);
//
//         let new_model = active_model.update(&self.db_connection).await?;
//         Ok(new_model)
//     }
// }
