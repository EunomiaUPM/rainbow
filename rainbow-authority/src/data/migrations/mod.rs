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

mod m20250403_094651_interaction;
mod m20250403_094651_issuing;
mod m20250403_094651_minions;
mod m20250403_094651_request;
mod m20250403_094651_verification;
use sea_orm_migration::prelude::*;

pub fn get_authority_migrations() -> Vec<Box<dyn MigrationTrait>> {
    vec![
        Box::new(m20250403_094651_request::Migration),
        Box::new(m20250403_094651_interaction::Migration),
        Box::new(m20250403_094651_verification::Migration),
        Box::new(m20250403_094651_issuing::Migration),
        Box::new(m20250403_094651_minions::Migration),
    ]
}

pub struct Migrator;
#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        get_authority_migrations()
    }
}
