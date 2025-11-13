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
use super::super::super::common::migrations::{m20250403_094651_authority_request, m20250403_094651_mates};
use super::{
    m20250403_094651_auth_interaction, m20250403_094651_auth_request, m20250403_094651_auth_token_requirements,
    m20250403_094651_auth_verification, m20250403_094651_business_mates
};
use sea_orm::prelude::async_trait;
use sea_orm_migration::{MigrationTrait, MigratorTrait};

pub fn get_auth_provider_migrations() -> Vec<Box<dyn MigrationTrait>> {
    vec![
        Box::new(m20250403_094651_auth_request::Migration),
        Box::new(m20250403_094651_auth_interaction::Migration),
        Box::new(m20250403_094651_auth_token_requirements::Migration),
        Box::new(m20250403_094651_auth_verification::Migration),
        Box::new(m20250403_094651_business_mates::Migration),
        Box::new(m20250403_094651_authority_request::Migration),
        Box::new(m20250403_094651_mates::Migration),
    ]
}
