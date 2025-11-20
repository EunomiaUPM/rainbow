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

use super::super::subtraits::{
    InteractionRepoTrait, IssuingRepoTrait, MinionsRepoTrait, RequestRepoTrait, VerificationRepoTrait,
};
use super::super::RepoTrait;
use super::repos::{AuthRequestRepo, AuthVerificationRepo, InteractionRepo, IssuingRepo, MinionsRepo};
use sea_orm::DatabaseConnection;
use std::sync::Arc;

#[derive(Clone)]
pub struct RepoForSql {
    request_repo: Arc<dyn RequestRepoTrait>,
    interaction_repo: Arc<dyn InteractionRepoTrait>,
    verification_repo: Arc<dyn VerificationRepoTrait>,
    issuing_repo: Arc<dyn IssuingRepoTrait>,
    minions_repo: Arc<dyn MinionsRepoTrait>,
}

impl RepoForSql {
    pub fn new(db_connection: DatabaseConnection) -> Self {
        Self {
            request_repo: Arc::new(AuthRequestRepo::new(db_connection.clone())),
            interaction_repo: Arc::new(InteractionRepo::new(db_connection.clone())),
            verification_repo: Arc::new(AuthVerificationRepo::new(db_connection.clone())),
            issuing_repo: Arc::new(IssuingRepo::new(db_connection.clone())),
            minions_repo: Arc::new(MinionsRepo::new(db_connection.clone())),
        }
    }
}

impl RepoTrait for RepoForSql {
    fn request(&self) -> Arc<dyn RequestRepoTrait> {
        self.request_repo.clone()
    }

    fn interaction(&self) -> Arc<dyn InteractionRepoTrait> {
        self.interaction_repo.clone()
    }

    fn verification(&self) -> Arc<dyn VerificationRepoTrait> {
        self.verification_repo.clone()
    }
    fn issuing(&self) -> Arc<dyn IssuingRepoTrait> {
        self.issuing_repo.clone()
    }

    fn minions(&self) -> Arc<dyn MinionsRepoTrait> {
        self.minions_repo.clone()
    }
}
