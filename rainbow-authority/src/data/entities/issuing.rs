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
use super::super::IntoActiveSet;
use crate::utils::create_opaque_token;
use sea_orm::entity::prelude::*;
use sea_orm::ActiveValue;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "issuing")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: String,
    pub name: String,
    pub pre_auth_code: String,
    pub tx_code: String,
    pub step: bool,
    pub vc_type: String,
    pub uri: String,
    pub token: String,
    pub aud: String,
    pub did: Option<String>,
    pub credential_id: String,
    pub credential: Option<String>
}

#[derive(Clone, Debug)]
pub struct NewModel {
    pub id: String,
    pub name: String,
    pub vc_type: String,
    pub uri: String,
    pub aud: String,
}

impl IntoActiveSet<ActiveModel> for NewModel {
    fn to_active(self) -> ActiveModel {
        let code = create_opaque_token();
        let tx_code = create_opaque_token();
        let token = create_opaque_token();
        let credential_id = uuid::Uuid::new_v4().to_string();
        ActiveModel {
            id: ActiveValue::Set(self.id),
            name: ActiveValue::Set(self.name),
            pre_auth_code: ActiveValue::Set(code),
            tx_code: ActiveValue::Set(tx_code),
            step: ActiveValue::Set(true),
            vc_type: ActiveValue::Set(self.vc_type),
            uri: ActiveValue::Set(self.uri),
            token: ActiveValue::Set(token),
            aud: ActiveValue::Set(self.aud),
            did: ActiveValue::Set(None),
            credential_id: ActiveValue::Set(credential_id),
            credential: ActiveValue::Set(None),
        }
    }
}

impl IntoActiveSet<ActiveModel> for Model {
    fn to_active(self) -> ActiveModel {
        ActiveModel {
            id: ActiveValue::Set(self.id),
            name: ActiveValue::Set(self.name),
            pre_auth_code: ActiveValue::Set(self.pre_auth_code),
            tx_code: ActiveValue::Set(self.tx_code),
            step: ActiveValue::Set(self.step),
            vc_type: ActiveValue::Set(self.vc_type),
            uri: ActiveValue::Set(self.uri),
            token: ActiveValue::Set(self.token),
            aud: ActiveValue::Set(self.aud),
            did: ActiveValue::Set(self.did),
            credential_id: ActiveValue::Set(self.credential_id),
            credential: ActiveValue::Set(self.credential),
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_one = "super::request::Entity")]
    Request,
    #[sea_orm(has_one = "super::verification::Entity")]
    Verification,
    #[sea_orm(has_one = "super::interaction::Entity")]
    Interaction,
}

impl Related<super::request::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Request.def()
    }
}

impl Related<super::verification::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Verification.def()
    }
}
impl Related<super::interaction::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Interaction.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
