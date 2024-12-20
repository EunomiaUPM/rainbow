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

use sea_orm::{DeriveActiveEnum, EnumIter};
use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_query::extension::postgres::Type;
use serde::{Deserialize, Serialize};

pub struct Migration;
impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20241111_000005_odrl_offers"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(Alias::new("entity_type"))
                    .values([
                        Alias::new("catalog"),
                        Alias::new("dataset"),
                        Alias::new("distribution"),
                        Alias::new("service"),
                    ])
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(ODRLOffers::Table)
                    .col(ColumnDef::new(ODRLOffers::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(ODRLOffers::ODRLOffers).json())
                    .col(ColumnDef::new(ODRLOffers::Entity).string().not_null())
                    .col(
                        ColumnDef::new(ODRLOffers::EntityType)
                            .custom(ODRLOffers::EntityType)
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(ODRLOffers::Table).to_owned()).await?;
        manager.drop_type(Type::drop().name(ODRLOffers::EntityType).if_exists().to_owned()).await
    }
}

#[derive(Iden)]
pub enum ODRLOffers {
    Table,
    Id,
    ODRLOffers,
    Entity,
    EntityType,
}

#[derive(EnumIter, DeriveActiveEnum, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "entity_type")]
pub enum EntityTypes {
    #[sea_orm(string_value = "catalog")]
    Catalog,
    #[sea_orm(string_value = "dataset")]
    Dataset,
    #[sea_orm(string_value = "distribution")]
    Distribution,
    #[sea_orm(string_value = "service")]
    DataService,
}
