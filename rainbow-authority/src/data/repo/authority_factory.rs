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
use super::basic_repo_trait::BasicRepoTrait;
use super::{auth, auth_interaction, auth_verification};

pub trait AuthorityRepoFactory: Send + Sync + Clone + 'static {
    fn create_repo(db_connection: DatabaseConnection) -> Box<dyn AuthorityRepoTrait>;
}

pub trait AuthorityRepoTrait: BasicRepoTrait<auth::Model> +BasicRepoTrait<auth_interaction::Model>+ BasicRepoTrait<auth_verification>+ Send + Sync + 'static {

}