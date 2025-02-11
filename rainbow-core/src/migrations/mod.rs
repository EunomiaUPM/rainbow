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

use rainbow_common::config::database::get_db_connection;
use rainbow_db::catalog::migrations::get_catalog_migrations;
use rainbow_db::contracts_consumer::migrations::get_contracts_migrations as get_consumer_migrations;
use rainbow_db::contracts_provider::migrations::get_contracts_migrations as get_provider_migrations;
use rainbow_db::dataplane::migrations::get_dataplane_migrations;
use rainbow_db::transfer_consumer::migrations::get_transfer_consumer_migrations;
use rainbow_db::transfer_provider::migrations::get_transfer_provider_migrations;

use sea_orm::prelude::async_trait;
use sea_orm_migration::{MigrationTrait, MigratorTrait};

pub struct ProviderMigrator;
#[async_trait::async_trait]
impl MigratorTrait for ProviderMigrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        let mut migrations = vec![];
        let mut provider_migrations = get_transfer_provider_migrations();
        let mut catalog_migrations = get_catalog_migrations();
        let mut dataplane_migrations = get_dataplane_migrations();
        let mut contract_migrations = get_provider_migrations();

        migrations.append(&mut provider_migrations);
        migrations.append(&mut catalog_migrations);
        migrations.append(&mut dataplane_migrations);
        migrations.append(&mut contract_migrations);
        migrations
    }
}

pub struct ConsumerMigrator;
#[async_trait::async_trait]
impl MigratorTrait for ConsumerMigrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        let mut migrations = vec![];
        let mut consumer_migrations = get_transfer_consumer_migrations();
        let mut dataplane_migrations = get_dataplane_migrations();
        let mut contract_migrations = get_consumer_migrations();

        migrations.append(&mut consumer_migrations);
        migrations.append(&mut dataplane_migrations);
        migrations.append(&mut contract_migrations);
        migrations
    }
}

pub async fn migrate_provider_db() -> anyhow::Result<()> {
    let db_connection = get_db_connection().await;
    ProviderMigrator::refresh(db_connection).await?;
    Ok(())
}

pub async fn migrate_consumer_db() -> anyhow::Result<()> {
    let db_connection = get_db_connection().await;
    ConsumerMigrator::refresh(db_connection).await?;
    Ok(())
}
