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

use sea_orm_migration::prelude::*;

pub struct Migration;
impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20241116_000001_transfer_processes"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TransferProcesses::Table)
                    .col(ColumnDef::new(TransferProcesses::ProviderPid).string().not_null().primary_key())
                    .col(ColumnDef::new(TransferProcesses::ConsumerPid).string())
                    .col(ColumnDef::new(TransferProcesses::AssociatedConsumer).string())
                    .col(ColumnDef::new(TransferProcesses::AgreementId).string())
                    .col(ColumnDef::new(TransferProcesses::DataPlaneId).string())
                    .col(ColumnDef::new(TransferProcesses::State).string().not_null())
                    .col(ColumnDef::new(TransferProcesses::StateAttribute).string())
                    .col(ColumnDef::new(TransferProcesses::CreatedAt).date_time().not_null())
                    .col(ColumnDef::new(TransferProcesses::UpdatedAt).date_time())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(TransferProcesses::Table).to_owned()).await
    }
}

#[derive(Iden)]
pub enum TransferProcesses {
    Table,
    ProviderPid,
    ConsumerPid,
    AgreementId,
    AssociatedConsumer,
    DataPlaneId,
    State,
    StateAttribute,
    CreatedAt,
    UpdatedAt,
}
