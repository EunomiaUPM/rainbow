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

use rainbow_common::data::IntoActiveSet;
use rand::distributions::Alphanumeric;
use rand::Rng;
use sea_orm::entity::prelude::*;
use sea_orm::ActiveValue;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "recv_verification")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: String, // REQUEST
    pub state: String,                     // RANDOM
    pub nonce: String,                     // RANDOM
    pub vc_type: String,                   // REQUEST
    pub audience: String,                  // SEMI-RANDOM
    pub holder: Option<String>,            // RESPONSE
    pub vpt: Option<String>,               // RESPONSE
    pub success: Option<bool>,             // RESPONSE
    pub status: String,                    // DEFAULT
    pub created_at: chrono::NaiveDateTime, // DEFAULT
    pub ended_at: Option<chrono::NaiveDateTime>, // RESPONSE
                                           // pub requirements: Value, TODO
}

#[derive(Clone, Debug)]
pub struct NewModel {
    pub id: String,       // REQUEST
    pub audience: String, // SEMI-RANDOM
    pub vc_type: String,                   // REQUEST
}

impl IntoActiveSet<ActiveModel> for NewModel {
    fn to_active(self) -> ActiveModel {
        let state: String = rand::thread_rng().sample_iter(&Alphanumeric).take(12).map(char::from).collect();
        let nonce: String = rand::thread_rng().sample_iter(&Alphanumeric).take(12).map(char::from).collect();
        let audience = format!("{}/{}", self.audience, &state);
        ActiveModel {
            id: ActiveValue::Set(self.id),
            state: ActiveValue::Set(state),
            nonce: ActiveValue::Set(nonce),
            vc_type: ActiveValue::Set(self.vc_type),
            audience: ActiveValue::Set(audience),
            holder: ActiveValue::Set(None),
            vpt: ActiveValue::Set(None),
            success: ActiveValue::Set(None),
            status: ActiveValue::Set("Pending".to_string()),
            created_at: ActiveValue::Set(chrono::Utc::now().naive_utc()),
            ended_at: ActiveValue::Set(None),
        }
    }
}

impl IntoActiveSet<ActiveModel> for Model {
    fn to_active(self) -> ActiveModel {
        ActiveModel {
            id: ActiveValue::Set(self.id),
            state: ActiveValue::Set(self.state),
            nonce: ActiveValue::Set(self.nonce),
            vc_type: ActiveValue::Set(self.vc_type),
            audience: ActiveValue::Set(self.audience),
            holder: ActiveValue::Set(self.holder),
            vpt: ActiveValue::Set(self.vpt),
            success: ActiveValue::Set(self.success),
            status: ActiveValue::Set(self.status),
            created_at: ActiveValue::Set(self.created_at),
            ended_at: ActiveValue::Set(self.ended_at),
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
