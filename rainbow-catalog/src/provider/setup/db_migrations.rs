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

use crate::provider::setup::config::CatalogApplicationProviderConfig;
use rainbow_common::config::provider_config::ApplicationProviderConfigTrait;
use rainbow_db::catalog::migrations::get_catalog_migrations;
use rainbow_db::events::migrations::get_events_migrations;
use sea_orm::Database;
use sea_orm_migration::{MigrationTrait, MigratorTrait};

pub struct CatalogMigration;

impl MigratorTrait for CatalogMigration {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        let mut migrations: Vec<Box<dyn MigrationTrait>> = vec![];
        let mut catalog_migrations = get_catalog_migrations();
        let mut pub_sub_migrations = get_events_migrations();

        migrations.append(&mut catalog_migrations);
        migrations.append(&mut pub_sub_migrations);
        migrations
    }
}

impl CatalogMigration {
    pub async fn run(config: &CatalogApplicationProviderConfig) -> anyhow::Result<()> {
        // db_connection
        let db_url = config.get_full_db_url();
        let db_connection = Database::connect(db_url).await.expect("Database can't connect");
        // run migration
        Self::refresh(&db_connection).await?;
        Ok(())
    }
}