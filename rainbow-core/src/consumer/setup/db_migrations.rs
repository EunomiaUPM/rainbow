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
use crate::consumer::setup::config::CoreApplicationConsumerConfig;
use anyhow::anyhow;
use rainbow_common::config::consumer_config::ApplicationConsumerConfigTrait;
use rainbow_common::config::ConfigRoles;
use rainbow_common::utils::get_urn;
use rainbow_db::auth_consumer::migrations::get_auth_consumer_migrations;
use rainbow_db::catalog::migrations::get_catalog_migrations;
use rainbow_db::contracts_consumer::migrations::get_contracts_migrations;
use rainbow_db::dataplane::migrations::get_dataplane_migrations;
use rainbow_db::events::migrations::get_events_migrations;
use rainbow_db::mates::entities::mates;
use rainbow_db::mates::migrations::get_mates_migrations;
use rainbow_db::transfer_consumer::migrations::get_transfer_consumer_migrations;
use sea_orm::sqlx::types::chrono;
use sea_orm::{ActiveValue, Database, DatabaseConnection, EntityTrait};
use sea_orm_migration::{MigrationTrait, MigratorTrait};

pub struct CoreConsumerMigration;

impl MigratorTrait for CoreConsumerMigration {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        let mut migrations: Vec<Box<dyn MigrationTrait>> = vec![];
        let mut transfer_provider_migrations = get_transfer_consumer_migrations();
        let mut catalog_migrations = get_catalog_migrations();
        let mut contract_negotiation_provider_migrations = get_contracts_migrations();
        let mut pub_sub_migrations = get_events_migrations();
        let mut auth_migrations = get_auth_consumer_migrations();
        let mut dataplane_migrations = get_dataplane_migrations();
        let mut mate_migrations = get_mates_migrations();

        migrations.append(&mut transfer_provider_migrations);
        migrations.append(&mut catalog_migrations);
        migrations.append(&mut contract_negotiation_provider_migrations);
        migrations.append(&mut pub_sub_migrations);
        migrations.append(&mut auth_migrations);
        migrations.append(&mut dataplane_migrations);
        migrations.append(&mut mate_migrations);
        migrations
    }
}

impl CoreConsumerMigration {
    pub async fn run(config: &CoreApplicationConsumerConfig) -> anyhow::Result<()> {
        // db_connection
        let db_url = config.get_full_db_url();
        let db_connection = Database::connect(db_url).await.expect("Database can't connect");
        // run migration
        Self::refresh(&db_connection).await?;
        // run seeders
        Self::seed_consumer_mate(&db_connection, config.get_ssi_auth_host_url().unwrap()).await?;
        Ok(())
    }
    async fn seed_consumer_mate(db: &DatabaseConnection, base_url: String) -> anyhow::Result<()> {
        let consumer = mates::ActiveModel {
            participant_id: ActiveValue::Set(get_urn(None).to_string()), // TODO PONER DID BIEN
            participant_slug: ActiveValue::Set("Consumer".to_string()),
            participant_type: ActiveValue::Set(ConfigRoles::Consumer.to_string()),
            base_url: ActiveValue::Set(Some(base_url)),
            token: ActiveValue::Set(None),
            token_actions: ActiveValue::Set(None),
            saved_at: ActiveValue::Set(chrono::Utc::now().naive_utc()),
            last_interaction: ActiveValue::Set(chrono::Utc::now().naive_utc()),
            is_me: ActiveValue::Set(true),
        };
        mates::Entity::insert(consumer)
            .exec(db)
            .await
            .map_err(|e| anyhow!(e))?;
        Ok(())
    }
}