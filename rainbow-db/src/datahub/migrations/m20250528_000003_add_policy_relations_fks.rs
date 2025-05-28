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

use crate::datahub::migrations::m20250528_000001_policy_relations::DatahubPolicyRelations;
use sea_orm_migration::prelude::*;

pub struct Migration;
impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20250528_000003_add_policy_relations_fks"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk-datahub_policy_relations-policy_template_id")
                    .from(DatahubPolicyRelations::Table, DatahubPolicyRelations::PolicyTemplateId)
                    .to(PolicyTemplates::Table, PolicyTemplates::Id)
                    .on_delete(ForeignKeyAction::Cascade) // Cascade deletion if a policy template is removed
                    .on_update(ForeignKeyAction::Cascade) // Cascade updates if a policy template ID changes
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("fk-datahub_policy_relations-policy_template_id")
                    .table(DatahubPolicyRelations::Table) // Specify the table from which the FK is dropped
                    .to_owned(),
            )
            .await?;
        Ok(())
    }
}

#[derive(Iden)]
pub enum PolicyTemplates {
    Table,
    Id,
    Title,
    Description,
    Content,
    CreatedAt,
}
