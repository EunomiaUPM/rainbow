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

mod m20241111_000001_catalog;
mod m20241111_000002_dataset;
mod m20241111_000003_distribution;
mod m20241111_000004_dataservice;
pub mod m20241111_000005_odrl_offers;

mod m20250718_000001_catalogrecord;
mod m20250718_000002_datasetseries;

mod m20250721_000001_resources;
mod m20250721_000002_themes;
mod m20250721_000003_keywords;
mod m20250721_000004_relations;
mod m20250721_000005_qualifiedrelations;
mod m20250721_000006_references;

mod m20250722_000001_foreign_keys;

pub fn get_catalog_migrations() -> Vec<Box<dyn MigrationTrait>> {
    vec![
        Box::new(m20241111_000001_catalog::Migration),
        Box::new(m20241111_000002_dataset::Migration),
        Box::new(m20241111_000003_distribution::Migration),
        Box::new(m20241111_000004_dataservice::Migration),
        Box::new(m20241111_000005_odrl_offers::Migration),

        Box::new(m20250718_000001_catalogrecord::Migration),
        Box::new(m20250718_000002_datasetseries::Migration),

        Box::new(m20250721_000003_keywords::Migration),
        Box::new(m20250721_000002_themes::Migration),
        Box::new(m20250721_000001_resources::Migration),
        Box::new(m20250721_000004_relations::Migration),
        Box::new(m20250721_000005_qualifiedrelations::Migration),
        Box::new(m20250721_000006_references::Migration),
        Box::new(m20250722_000001_foreign_keys::Migration),
    ]
}
pub fn get_datahub_catalog_migrations() -> Vec<Box<dyn MigrationTrait>> {
    vec![
        Box::new(m20241111_000005_odrl_offers::Migration),
    ]
}
pub struct Migrator;
#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        get_catalog_migrations()
    }
}
