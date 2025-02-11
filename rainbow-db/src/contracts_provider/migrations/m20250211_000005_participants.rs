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
use serde_json::json;

pub struct Migration;
impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20250211_000005_participants"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Participants::Table)
                    .col(
                        ColumnDef::new(Participants::ParticipantId)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Participants::IdentityToken).string().not_null())
                    .col(ColumnDef::new(Participants::Type).string().not_null())
                    .col(ColumnDef::new(Participants::ExtraFields).json().default(json!({})))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Participants::Table).to_owned()).await
    }
}

#[derive(Iden)]
pub enum Participants {
    Table,
    ParticipantId,
    IdentityToken,
    Type,
    ExtraFields,
}
