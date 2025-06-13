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
use rainbow_common::protocol::contract::contract_odrl::OdrlOffer;
use rainbow_common::protocol::contract::odrloffer_wrapper::OdrlOfferWrapper;
use sea_orm::Value;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, serde::Serialize)]
#[sea_orm(table_name = "data_hub_policy_relations")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: String,
    pub dataset_id: String,
    pub policy_template_id: String,
    pub odrl_offer: OdrlOfferWrapper,
    pub created_at: chrono::NaiveDateTime,
}


#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // Relación con el template
    #[sea_orm(
        belongs_to = "super::policy_templates::Entity",
        from = "Column::PolicyTemplateId",
        to = "super::policy_templates::Column::Id"
    )]
    PolicyTemplate,
}

impl Related<super::policy_templates::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PolicyTemplate.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
