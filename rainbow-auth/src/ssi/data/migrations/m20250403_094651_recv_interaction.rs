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

use sea_orm_migration::prelude::*;

pub struct Migration;
impl MigrationName for Migration {
    fn name(&self) -> &str { "m20250403_094651_recv_interaction" }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(RecvInteraction::Table)
                    .col(ColumnDef::new(RecvInteraction::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(RecvInteraction::Start).array(ColumnType::Text).not_null())
                    .col(ColumnDef::new(RecvInteraction::Method).string().not_null())
                    .col(ColumnDef::new(RecvInteraction::Uri).string().not_null())
                    .col(ColumnDef::new(RecvInteraction::ClientNonce).string().not_null())
                    .col(ColumnDef::new(RecvInteraction::HashMethod).string().not_null())
                    .col(ColumnDef::new(RecvInteraction::Hints).string())
                    .col(ColumnDef::new(RecvInteraction::GrantEndpoint).string().not_null())
                    .col(ColumnDef::new(RecvInteraction::ContinueEndpoint).string().not_null())
                    .col(ColumnDef::new(RecvInteraction::ContinueId).string().not_null())
                    .col(ColumnDef::new(RecvInteraction::ContinueToken).string().not_null())
                    .col(ColumnDef::new(RecvInteraction::ASNonce).string())
                    .col(ColumnDef::new(RecvInteraction::InteractRef).string())
                    .col(ColumnDef::new(RecvInteraction::Hash).string())
                    .to_owned()
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(RecvInteraction::Table).to_owned()).await
    }
}

#[derive(Iden)]
pub enum RecvInteraction {
    Table,
    Id,
    Start,
    Method,
    Uri,
    ClientNonce,
    ASNonce,
    InteractRef,
    GrantEndpoint,
    ContinueEndpoint,
    ContinueId,
    ContinueToken,
    Hash,
    HashMethod,
    Hints
}
