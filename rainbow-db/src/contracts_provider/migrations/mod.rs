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
pub mod m20250211_000001_cn_processes;
pub mod m20250211_000002_cn_messages;
pub mod m20250211_000003_cn_offers;
pub mod m20250211_000004_agreements;
pub mod m20250211_000005_participants;

pub fn get_contracts_migrations() -> Vec<Box<dyn MigrationTrait>> {
    vec![
        Box::new(m20250211_000005_participants::Migration),
        Box::new(m20250211_000001_cn_processes::Migration),
        Box::new(m20250211_000002_cn_messages::Migration),
        Box::new(m20250211_000003_cn_offers::Migration),
        Box::new(m20250211_000004_agreements::Migration),
    ]
}

pub struct Migrator;
#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        get_contracts_migrations()
    }
}
