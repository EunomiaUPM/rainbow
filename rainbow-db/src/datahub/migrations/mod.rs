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

mod m20250528_000001_policy_relations;
mod m20250528_000002_policy_templates;
mod m20250528_000003_add_policy_relations_fks;

pub fn get_datahub_migrations() -> Vec<Box<dyn MigrationTrait>> {
    vec![
        Box::new(m20250528_000001_policy_relations::Migration),
        Box::new(m20250528_000002_policy_templates::Migration),
        Box::new(m20250528_000003_add_policy_relations_fks::Migration),
    ]
}
pub struct Migrator;
#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        get_datahub_migrations()
    }
}
