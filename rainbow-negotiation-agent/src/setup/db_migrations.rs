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

use crate::data::migrations::get_negotiation_agent_migrations;
use rainbow_common::config::global_config::ApplicationGlobalConfig;
use sea_orm::Database;
use sea_orm_migration::{MigrationTrait, MigratorTrait};

pub struct NegotiationAgentMigration;

impl MigratorTrait for NegotiationAgentMigration {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        let mut migrations: Vec<Box<dyn MigrationTrait>> = vec![];
        let mut transfer_agent_migrations = get_negotiation_agent_migrations();
        migrations.append(&mut transfer_agent_migrations);
        migrations
    }
}

impl NegotiationAgentMigration {
    pub async fn run(config: &ApplicationGlobalConfig) -> anyhow::Result<()> {
        // db_connection
        let db_url = config.database_config.as_db_url();
        let db_connection = Database::connect(db_url).await.expect("Database can't connect");
        // run migration
        Self::refresh(&db_connection).await?;
        Ok(())
    }
}
