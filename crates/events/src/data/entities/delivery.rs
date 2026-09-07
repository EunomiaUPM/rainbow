/*
 * Copyright (C) 2026 - Universidad Politécnica de Madrid - UPM
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 */

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "event_deliveries")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: String,
    pub event_id: String,
    pub subscription_id: String,
    pub status: String,
    pub attempts: i32,
    pub last_attempt_at: Option<chrono::NaiveDateTime>,
    pub next_retry_at: Option<chrono::NaiveDateTime>,
    pub error_message: Option<String>,
    pub response_status_code: Option<i32>,
    pub delivered_at: Option<chrono::NaiveDateTime>,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::event::Entity",
        from = "Column::EventId",
        to = "super::event::Column::Id"
    )]
    Event,
    #[sea_orm(
        belongs_to = "super::subscription::Entity",
        from = "Column::SubscriptionId",
        to = "super::subscription::Column::Id"
    )]
    Subscription,
}

impl Related<super::event::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Event.def()
    }
}

impl Related<super::subscription::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Subscription.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
