/*
 * Copyright (C) 2025 - Universidad Politécnica de Madrid - UPM
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use chrono;
use rainbow_common::data::IntoActiveSet;
use sea_orm::entity::prelude::*;
use sea_orm::{ActiveValue, DeriveEntityModel};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "business_mates")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: String,
    pub participant_id: String,
    pub token: Option<String>,
    pub saved_at: chrono::NaiveDateTime,
    pub last_interaction: chrono::NaiveDateTime,
}

#[derive(Clone, Debug)]
pub struct NewModel {
    pub id: String,
    pub participant_id: String,
    pub token: Option<String>,
}

impl IntoActiveSet<ActiveModel> for NewModel {
    fn to_active(self) -> ActiveModel {
        ActiveModel {
            id: ActiveValue::Set(self.id),
            participant_id: ActiveValue::Set(self.participant_id),
            token: ActiveValue::Set(self.token),
            saved_at: ActiveValue::Set(chrono::Utc::now().naive_utc()),
            last_interaction: ActiveValue::Set(chrono::Utc::now().naive_utc()),
        }
    }
}

impl IntoActiveSet<ActiveModel> for Model {
    fn to_active(self) -> ActiveModel {
        ActiveModel {
            id: ActiveValue::Set(self.id),
            participant_id: ActiveValue::Set(self.participant_id),
            token: ActiveValue::Set(self.token),
            saved_at: ActiveValue::Set(self.saved_at),
            last_interaction: ActiveValue::Set(chrono::Utc::now().naive_utc()),
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
