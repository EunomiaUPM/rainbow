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

use crate::core::{DataPlanePeer, DataPlanePeerCreationBehavior, PersistModel};
use crate::data::entities::{data_plane_field, data_plane_process};
use axum::async_trait;
use rainbow_common::config::config::ConfigRoles;
use rainbow_common::dcat_formats::{FormatAction, FormatProtocol};
use sea_orm::{ActiveValue, ColumnTrait, DbConn, EntityTrait, QueryFilter};
use std::collections::HashMap;
use uuid::Uuid;

pub mod implementation;

pub struct HttpDataPlane {
    pub inner: DataPlanePeer,
}

#[async_trait]
impl PersistModel<data_plane_process::Model> for HttpDataPlane {
    async fn persist(self, db_connection: &DbConn) -> anyhow::Result<Box<Self>> {
        let dp = data_plane_process::Entity::find_by_id(self.inner.id).one(db_connection).await?;
        let attributes = data_plane_field::Entity::find()
            .filter(data_plane_field::Column::DataPlaneProcessId.eq(self.inner.id))
            .all(db_connection)
            .await?;

        if let Some(dp) = dp {
            let dp_model = data_plane_process::Entity::update(data_plane_process::ActiveModel {
                id: ActiveValue::Set(dp.id),
                role: ActiveValue::Set(dp.role),
                address: ActiveValue::Set(self.inner.local_address.clone().unwrap()),
                dct_action_format: ActiveValue::Set(dp.dct_action_format),
                dct_action_protocol: ActiveValue::Set(dp.dct_action_protocol),
                created_at: ActiveValue::Set(dp.created_at),
                updated_at: ActiveValue::Set(Option::from(chrono::Utc::now().naive_utc())),
            })
            .exec(db_connection)
            .await?;

            for (key, value) in &self.inner.attributes {
                let exists = attributes
                    .iter()
                    .any(|attr| attr.key == key.to_string() && attr.value == value.to_string());
                if !exists {
                    data_plane_field::Entity::insert(data_plane_field::ActiveModel {
                        id: ActiveValue::Set(Uuid::new_v4()),
                        key: ActiveValue::Set(key.to_owned()),
                        value: ActiveValue::Set(value.to_owned()),
                        data_plane_process_id: ActiveValue::Set(self.inner.id),
                    })
                    .exec(db_connection)
                    .await?;
                }
            }

            for attribute in attributes {
                let exists = &self.inner.attributes.iter().any(|attr| {
                    attr.0.to_string() == attribute.key && attr.1.to_string() == attribute.value
                });
                if !exists {
                    data_plane_field::Entity::delete(data_plane_field::ActiveModel {
                        id: ActiveValue::Set(attribute.id),
                        ..Default::default()
                    })
                    .exec(db_connection)
                    .await?;
                }
            }
        } else {
            data_plane_process::Entity::insert(data_plane_process::ActiveModel {
                id: ActiveValue::Set(self.inner.id),
                role: ActiveValue::Set(self.inner.role.to_string()),
                address: ActiveValue::Set(self.inner.local_address.clone().unwrap()),
                dct_action_format: ActiveValue::Set(self.inner.dct_formats.action.to_string()),
                dct_action_protocol: ActiveValue::Set(self.inner.dct_formats.protocol.to_string()),
                created_at: ActiveValue::Set(chrono::Utc::now().naive_utc()),
                updated_at: ActiveValue::Set(None),
            })
            .exec(db_connection)
            .await?;

            for (key, value) in &self.inner.attributes {
                data_plane_field::Entity::insert(data_plane_field::ActiveModel {
                    id: ActiveValue::Set(Uuid::new_v4()),
                    key: ActiveValue::Set(key.to_string()),
                    value: ActiveValue::Set(value.to_string()),
                    data_plane_process_id: ActiveValue::Set(self.inner.id.clone()),
                })
                .exec(db_connection)
                .await?;
            }
        }
        Ok(Box::new(self))
    }
}

impl DataPlanePeerCreationBehavior for HttpDataPlane {
    fn create_data_plane_peer() -> Self {
        Self { inner: DataPlanePeer::default() }
    }

    fn create_data_plane_peer_from_inner(inner: DataPlanePeer) -> Self {
        Self { inner }
    }

    fn with_role(mut self, role: ConfigRoles) -> Self {
        self.inner.role = role;
        self
    }

    fn with_local_address(mut self, local_address: String) -> Self {
        self.inner.local_address = Some(local_address);
        self
    }

    fn with_attributes(mut self, attributes: HashMap<String, String>) -> Self {
        self.inner.attributes.extend(attributes);
        self
    }

    fn add_attribute(mut self, key: String, value: String) -> Self {
        self.inner.attributes.insert(key, value);
        self
    }

    fn delete_attribute(mut self, key: String) -> Self {
        self.inner.attributes.remove(&key);
        self
    }

    fn with_action(mut self, action: FormatAction) -> Self {
        self.inner.dct_formats.action = action;
        self
    }

    fn with_protocol(mut self, protocol: FormatProtocol) -> Self {
        self.inner.dct_formats.protocol = protocol;
        self
    }
}
