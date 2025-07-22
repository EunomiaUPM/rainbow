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

use crate::contracts_consumer::migrations::m20250211_000002_cn_messages::CNMessages;
use sea_orm_migration::prelude::*;

pub struct Migration;
impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20250211_000004_agreements"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Agreements::Table)
                    .col(ColumnDef::new(Agreements::AgreementId).string().not_null().primary_key())
                    .col(ColumnDef::new(Agreements::ConsumerParticipantId).string().not_null())
                    .col(ColumnDef::new(Agreements::ProviderParticipantId).string().not_null())
                    .col(ColumnDef::new(Agreements::CnMessageId).string().not_null())
                    .col(ColumnDef::new(Agreements::AgreementContent).json().not_null())
                    .col(ColumnDef::new(Agreements::CreatedAt).date_time().not_null())
                    .col(ColumnDef::new(Agreements::Active).boolean().not_null().default(true))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_agreements_cn_messages")
                            .from(Agreements::Table, Agreements::CnMessageId)
                            .to(CNMessages::Table, CNMessages::CnMessageId)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Agreements::Table).to_owned()).await
    }
}

#[derive(Iden)]
pub enum Agreements {
    Table,
    AgreementId,
    ConsumerParticipantId,
    ProviderParticipantId,
    CnMessageId,
    AgreementContent,
    CreatedAt,
    Active,
}
