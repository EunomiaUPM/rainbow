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

use super::super::AuthConsumerRepoTrait;
use crate::ssi::common::services::repo::repos::{
    MatesRepo, ReqInteractionRepo, ReqVcRepo, ReqVerificationRepo, TokenRequirementsRepo,
};
use crate::ssi::common::services::repo::subtraits::{
    MatesTrait, ReqInteractionTrait, ReqVcTrait, ReqVerificationTrait, TokenRequirementsTrait,
};
use crate::ssi::consumer::services::repo::postgres::repos::ReqRequestRepo;
use crate::ssi::consumer::services::repo::subtraits::ReqRequestTrait;
use sea_orm::DatabaseConnection;
use std::sync::Arc;

pub struct AuthConsumerRepoForSql {
    request_repo: Arc<dyn ReqRequestTrait>,
    interaction_repo: Arc<dyn ReqInteractionTrait>,
    verification_repo: Arc<dyn ReqVerificationTrait>,
    token_req_repo: Arc<dyn TokenRequirementsTrait>,
    mates_repo: Arc<dyn MatesTrait>,
    vc_req_repo: Arc<dyn ReqVcTrait>,
}

impl AuthConsumerRepoForSql {
    pub fn create_repo(db_connection: DatabaseConnection) -> Self {
        Self {
            request_repo: Arc::new(ReqRequestRepo::new(db_connection.clone())),
            interaction_repo: Arc::new(ReqInteractionRepo::new(db_connection.clone())),
            verification_repo: Arc::new(ReqVerificationRepo::new(db_connection.clone())),
            token_req_repo: Arc::new(TokenRequirementsRepo::new(db_connection.clone())),
            mates_repo: Arc::new(MatesRepo::new(db_connection.clone())),
            vc_req_repo: Arc::new(ReqVcRepo::new(db_connection.clone())),
        }
    }
}

impl AuthConsumerRepoTrait for AuthConsumerRepoForSql {
    fn request_req(&self) -> Arc<dyn ReqRequestTrait> {
        self.request_repo.clone()
    }

    fn interaction_req(&self) -> Arc<dyn ReqInteractionTrait> {
        self.interaction_repo.clone()
    }

    fn verification_req(&self) -> Arc<dyn ReqVerificationTrait> {
        self.verification_repo.clone()
    }

    fn token_requirements(&self) -> Arc<dyn TokenRequirementsTrait> {
        self.token_req_repo.clone()
    }

    fn mates(&self) -> Arc<dyn MatesTrait> {
        self.mates_repo.clone()
    }

    fn vc_req(&self) -> Arc<dyn ReqVcTrait> {
        self.vc_req_repo.clone()
    }
}
