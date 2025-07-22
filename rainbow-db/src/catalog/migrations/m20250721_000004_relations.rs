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

use super::m20250721_000001_resources::Resources;
use sea_orm_migration::prelude::*;

pub struct Migration;
impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20252107_000004_relations"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Relations::Table)
                    .col(ColumnDef::new(Relations::DcatRelationship).string().not_null())
                    .col(ColumnDef::new(Relations::DcatResource1).string().not_null())
                    .col(ColumnDef::new(Relations::DcatResource2).string().not_null())
                    // .foreign_key(
                    //     ForeignKey::create()
                    //         .name("fk_relations_resource1")
                    //         .from(Relations::Table, Relations::DcatResource1)
                    //         .to(Resources::Table, Resources::ResourceId)
                    //         .on_delete(ForeignKeyAction::Cascade)
                    // )
                    // .foreign_key(
                    //     ForeignKey::create()
                    //         .name("fk_relations_resource2")
                    //         .from(Relations::Table, Relations::DcatResource2)
                    //         .to(Resources::Table, Resources::ResourceId)
                    //         .on_delete(ForeignKeyAction::Cascade)
                    // )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Resources::Table).to_owned()).await
    }
}

#[derive(Iden)]
pub enum Relations {
    Table,
    DcatRelationship,
    DcatResource1,
    DcatResource2,
}