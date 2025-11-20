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
use sea_orm::DatabaseConnection;
use rainbow_common::db::BasicRepoTrait;
use crate::ssi::common::data::entities::token_requirements::{Entity, Model};
use super::super::subtraits::TokenRequirementsTrait;

pub struct TokenRequirementsRepo {
    db_connection: DatabaseConnection,
}

impl TokenRequirementsRepo {
    pub fn new(db_connection: DatabaseConnection) -> Self {
        Self { db_connection }
    }
}

impl BasicRepoTrait<Entity, Model> for TokenRequirementsRepo {
    fn db(&self) -> &DatabaseConnection {
        &self.db_connection
    }
}

impl TokenRequirementsTrait for TokenRequirementsRepo {

}