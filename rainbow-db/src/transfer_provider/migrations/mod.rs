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

mod m20241116_000001_transfer_process;
mod m20241116_000002_transfer_messages;
mod m20241116_000003_contract_agreements;

pub fn get_transfer_provider_migrations() -> Vec<Box<dyn MigrationTrait>> {
    vec![
        Box::new(m20241116_000001_transfer_process::Migration),
        Box::new(m20241116_000002_transfer_messages::Migration),
        Box::new(m20241116_000003_contract_agreements::Migration),
    ]
}

pub struct Migrator;
#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        get_transfer_provider_migrations()
    }
}
