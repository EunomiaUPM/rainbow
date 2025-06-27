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
        "m20250403_094651_busmates"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(BusMates::Table)
                    .col(ColumnDef::new(BusMates::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(BusMates::ParticipantId).string().not_null())
                    .col(ColumnDef::new(BusMates::ParticipantType).string().not_null())
                    .col(ColumnDef::new(BusMates::Token).string())
                    .col(ColumnDef::new(BusMates::TokenActions).string())
                    .col(ColumnDef::new(BusMates::SavedAt).date_time().not_null())
                    .col(ColumnDef::new(BusMates::LastInteraction).date_time().not_null())
                    .col(ColumnDef::new(BusMates::IsMe).boolean().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(BusMates::Table).to_owned()).await
    }
}

#[derive(Iden)]
pub enum BusMates {
    Table,
    Id,
    ParticipantId,
    ParticipantType,
    Token,
    TokenActions,
    SavedAt,
    LastInteraction,
    IsMe,
}
