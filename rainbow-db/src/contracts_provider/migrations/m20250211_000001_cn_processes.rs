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

use crate::contracts_consumer::migrations::m20250211_000001_cn_processes::CnProcesses;
use sea_orm_migration::prelude::*;

pub struct Migration;
impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20250211_000001_cn_processes"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(CNProcesses::Table)
                    .col(ColumnDef::new(CNProcesses::ProviderId).string().not_null().primary_key())
                    .col(ColumnDef::new(CNProcesses::ConsumerId).string())
                    .col(ColumnDef::new(CNProcesses::AssociatedConsumer).string())
                    .col(ColumnDef::new(CnProcesses::IsBusiness).boolean())
                    .col(ColumnDef::new(CNProcesses::State).string())
                    .col(ColumnDef::new(CNProcesses::InitiatedBy).string())
                    .col(ColumnDef::new(CNProcesses::CreatedAt).date_time().not_null())
                    .col(ColumnDef::new(CNProcesses::UpdatedAt).date_time())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(CNProcesses::Table).to_owned()).await
    }
}

#[derive(Iden)]
pub enum CNProcesses {
    Table,
    ProviderId,
    ConsumerId,
    AssociatedConsumer,
    IsBusiness,
    State,
    InitiatedBy,
    CreatedAt,
    UpdatedAt,
}
