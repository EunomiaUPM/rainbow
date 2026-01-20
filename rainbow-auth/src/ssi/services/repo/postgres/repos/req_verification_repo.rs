/*
 * Copyright (C) 2025 - Universidad Politécnica de Madrid - UPM
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use rainbow_common::data::BasicRepoTrait;
use sea_orm::DatabaseConnection;

use crate::ssi::data::entities::req_verification::{Entity, NewModel};
use crate::ssi::services::repo::subtraits::ReqVerificationTrait;

pub struct ReqVerificationRepo {
    db_connection: DatabaseConnection,
}

impl ReqVerificationRepo {
    pub fn new(db_connection: DatabaseConnection) -> Self {
        Self { db_connection }
    }
}

impl BasicRepoTrait<Entity, NewModel> for ReqVerificationRepo {
    fn db(&self) -> &DatabaseConnection {
        &self.db_connection
    }
}

impl ReqVerificationTrait for ReqVerificationRepo {}
