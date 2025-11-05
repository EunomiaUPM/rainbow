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

use super::repos::{AuthInteractionRepo, AuthRequestRepo, AuthVerificationRepo, MinionsRepo};
use super::traits::{AuthInteractionRepoTrait, AuthRequestRepoTrait, AuthVerificationRepoTrait, MinionsRepoTrait};
use super::RepoFactoryTrait;
use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct RepoForSql<RQ, IN, VE, MI>
where
    RQ: AuthRequestRepoTrait + Send + Sync + Clone + 'static,
    IN: AuthInteractionRepoTrait + Send + Sync + Clone + 'static,
    VE: AuthVerificationRepoTrait + Send + Sync + Clone + 'static,
    MI: MinionsRepoTrait + Send + Sync + Clone + 'static,
{
    request_repo: RQ,
    interaction_repo: IN,
    verification_repo: VE,
    minions_repo: MI,
}

impl RepoForSql<AuthRequestRepo, AuthInteractionRepo, AuthVerificationRepo, MinionsRepo> {
    pub fn new(db_connection: DatabaseConnection) -> Self {
        Self {
            request_repo: AuthRequestRepo::new(db_connection.clone()),
            interaction_repo: AuthInteractionRepo::new(db_connection.clone()),
            verification_repo: AuthVerificationRepo::new(db_connection.clone()),
            minions_repo: MinionsRepo::new(db_connection.clone()),
        }
    }
}

impl RepoFactoryTrait for RepoForSql<AuthRequestRepo, AuthInteractionRepo, AuthVerificationRepo, MinionsRepo> {
    type RequestRepo = AuthRequestRepo;
    type InteractionRepo = AuthInteractionRepo;
    type VerificationRepo = AuthVerificationRepo;
    type MinionsRepo = MinionsRepo;

    fn request(&self) -> &Self::RequestRepo {
        &self.request_repo
    }

    fn interaction(&self) -> &Self::InteractionRepo {
        &self.interaction_repo
    }

    fn verification(&self) -> &Self::VerificationRepo {
        &self.verification_repo
    }

    fn minions(&self) -> &Self::MinionsRepo {
        &self.minions_repo
    }
}
