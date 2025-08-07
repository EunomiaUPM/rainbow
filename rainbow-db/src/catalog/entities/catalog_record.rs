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
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "catalog_records")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: String,
    pub dcat_catalog: String,
    pub dct_title: String,
    pub dct_description: String,
    pub dct_issued: chrono::NaiveDateTime,
    pub foaf_primary_topic: String
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::catalog::Entity",
        from = "Column::DcatCatalog",
        to = "super::catalog::Column::Id"
    )]
    Catalog,
}


impl Related<super::catalog::Entity> for ActiveModel {
    fn to() -> RelationDef {
        Relation::Catalog.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}