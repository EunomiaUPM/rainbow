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
use super::factory_trait::AuthConsumerRepoTrait;
use super::repos::{
    AuthInteractionConsumerRepo, AuthRequestConsumerRepo, AuthTokenRequirementsConsumerRepo,
    AuthVerificationConsumerRepo,
};
use super::traits::{
    AuthInteractionRepoTrait, AuthRequestRepoTrait, AuthTokenRequirementsRepoTrait, AuthVerificationRepoTrait,
};
use crate::auth::common::repos::{AuthorityRequestRepo, MatesRepo};
use crate::auth::common::traits::{AuthorityRequestRepoTrait, MatesRepoTrait};
use sea_orm::DatabaseConnection;
use std::sync::Arc;

#[derive(Clone)]
pub struct AuthConsumerRepoForSql {
    request_repo: Arc<dyn AuthRequestRepoTrait>,
    interaction_repo: Arc<dyn AuthInteractionRepoTrait>,
    verification_repo: Arc<dyn AuthVerificationRepoTrait>,
    token_req_repo: Arc<dyn AuthTokenRequirementsRepoTrait>,
    mates_repo: Arc<dyn MatesRepoTrait>,
    authority_repo: Arc<dyn AuthorityRequestRepoTrait>,
}

impl AuthConsumerRepoForSql {
    pub fn create_repo(db_connection: DatabaseConnection) -> Self {
        Self {
            request_repo: Arc::new(AuthRequestConsumerRepo::new(db_connection.clone())),
            interaction_repo: Arc::new(AuthInteractionConsumerRepo::new(db_connection.clone())),
            verification_repo: Arc::new(AuthVerificationConsumerRepo::new(db_connection.clone())),
            token_req_repo: Arc::new(AuthTokenRequirementsConsumerRepo::new(
                db_connection.clone(),
            )),
            mates_repo: Arc::new(MatesRepo::new(db_connection.clone())),
            authority_repo: Arc::new(AuthorityRequestRepo::new(db_connection.clone())),
        }
    }
}

impl AuthConsumerRepoTrait for AuthConsumerRepoForSql {
    fn request(&self) -> Arc<dyn AuthRequestRepoTrait> {
        self.request_repo.clone()
    }

    fn interaction(&self) -> Arc<dyn AuthInteractionRepoTrait> {
        self.interaction_repo.clone()
    }

    fn verification(&self) -> Arc<dyn AuthVerificationRepoTrait> {
        self.verification_repo.clone()
    }

    fn token_requirements(&self) -> Arc<dyn AuthTokenRequirementsRepoTrait> {
        self.token_req_repo.clone()
    }

    fn mates(&self) -> Arc<dyn MatesRepoTrait> {
        self.mates_repo.clone()
    }

    fn authority(&self) -> Arc<dyn AuthorityRequestRepoTrait> {
        self.authority_repo.clone()
    }
}
