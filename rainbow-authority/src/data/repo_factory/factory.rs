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

use super::factory_trait::AuthRepoFactoryTrait;
use super::repos::{AuthInteractionRepo, AuthRequestRepo, AuthVerificationRepo, MinionsRepo};
use super::traits::{AuthInteractionRepoTrait, AuthRequestRepoTrait, AuthVerificationRepoTrait, MinionsRepoTrait};
use sea_orm::DatabaseConnection;
use std::sync::Arc;

#[derive(Clone)]
pub struct AuthRepoForSql {
    request_repo: Arc<dyn AuthRequestRepoTrait>,
    interaction_repo: Arc<dyn AuthInteractionRepoTrait>,
    verification_repo: Arc<dyn AuthVerificationRepoTrait>,
    minions_repo: Arc<dyn MinionsRepoTrait>,
}

impl AuthRepoForSql {
    pub fn create_repo(db_connection: DatabaseConnection) -> Self {
        Self {
            request_repo: Arc::new(AuthRequestRepo::new(db_connection.clone())),
            interaction_repo: Arc::new(AuthInteractionRepo::new(db_connection.clone())),
            verification_repo: Arc::new(AuthVerificationRepo::new(db_connection.clone())),
            minions_repo: Arc::new(MinionsRepo::new(db_connection.clone())),
        }
    }
}

impl AuthRepoFactoryTrait for AuthRepoForSql {
    fn request(&self) -> Arc<dyn AuthRequestRepoTrait> {
        self.request_repo.clone()
    }

    fn interaction(&self) -> Arc<dyn AuthInteractionRepoTrait> {
        self.interaction_repo.clone()
    }

    fn verification(&self) -> Arc<dyn AuthVerificationRepoTrait> {
        self.verification_repo.clone()
    }

    fn minions(&self) -> Arc<dyn MinionsRepoTrait> {
        self.minions_repo.clone()
    }
}
