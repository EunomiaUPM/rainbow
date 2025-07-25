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

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "auth_verification")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: String,
    pub status: String,
    pub scheme: String,
    pub response_type: String,
    pub client_id: String,
    pub response_mode: String,
    pub pd_uri: String,
    pub client_id_scheme: String,
    pub nonce: String,
    pub response_uri: String,
    pub uri: String,
    pub created_at: chrono::NaiveDateTime,
    pub ended_at: Option<chrono::NaiveDateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_one = "super::auth::Entity")]
    Auth,
}

impl Related<super::auth::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Auth.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
