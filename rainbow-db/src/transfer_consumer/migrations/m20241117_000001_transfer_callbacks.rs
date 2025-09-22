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

use sea_orm_migration::prelude::*;

pub struct Migration;
impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20241117_000001_transfer_callbacks"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TransferCallbacks::Table)
                    .col(ColumnDef::new(TransferCallbacks::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(TransferCallbacks::ConsumerPid).string().not_null())
                    .col(ColumnDef::new(TransferCallbacks::ProviderPid).string())
                    .col(ColumnDef::new(TransferCallbacks::AssociatedProvider).string())
                    .col(ColumnDef::new(TransferCallbacks::CreatedAt).date_time().not_null())
                    .col(ColumnDef::new(TransferCallbacks::UpdatedAt).date_time())
                    .col(ColumnDef::new(TransferCallbacks::DataPlaneId).string())
                    .col(ColumnDef::new(TransferCallbacks::DataAddress).json())
                    .col(ColumnDef::new(TransferCallbacks::RestartFlag).boolean().not_null().default(false))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(TransferCallbacks::Table).to_owned()).await
    }
}

#[derive(Iden)]
pub enum TransferCallbacks {
    Table,
    Id,
    ProviderPid,
    ConsumerPid,
    AssociatedProvider,
    DataAddress,
    CreatedAt,
    UpdatedAt,
    DataPlaneId,
    RestartFlag,
}
