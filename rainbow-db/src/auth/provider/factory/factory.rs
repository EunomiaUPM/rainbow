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

use super::factory_trait::AuthProviderRepoTrait;
use super::repos::{
    BusinessMatesProviderRepo, RecvInteractionRepo, RecvRequestRepo, RecvTokenRequirementsRepo, RecvVerificationRepo,
};
use super::traits::{
    BusinessMatesRepoTrait, RecvInteractionTrait, RecvRequestTrait, RecvTokenRequirementsTrait, RecvVerificationTrait,
};
use crate::auth::common::repos::{MatesRepo, ReqInteractionRepo, ReqVerificationRepo, VcRequestRepo};
use crate::auth::common::traits::{MatesTrait, ReqInteractionTrait, ReqVcTrait, ReqVerificationTrait};
use sea_orm::DatabaseConnection;
use std::sync::Arc;

#[derive(Clone)]
pub struct AuthProviderRepoForSql {
    request_repo: Arc<dyn RecvRequestTrait>,
    interaction_rcvr_repo: Arc<dyn RecvInteractionTrait>,
    interaction_req_repo: Arc<dyn ReqInteractionTrait>,
    verification_rcvr_repo: Arc<dyn RecvVerificationTrait>,
    verification_req_repo: Arc<dyn ReqVerificationTrait>,
    token_req_repo: Arc<dyn RecvTokenRequirementsTrait>,
    authority_repo: Arc<dyn ReqVcTrait>,
    mates_repo: Arc<dyn MatesTrait>,
    business_mates_repo: Arc<dyn BusinessMatesRepoTrait>,
}

impl AuthProviderRepoForSql {
    pub fn create_repo(db_connection: DatabaseConnection) -> Self {
        Self {
            request_repo: Arc::new(RecvRequestRepo::new(db_connection.clone())),
            interaction_rcvr_repo: Arc::new(RecvInteractionRepo::new(db_connection.clone())),
            interaction_req_repo: Arc::new(ReqInteractionRepo::new(db_connection.clone())),
            verification_rcvr_repo: Arc::new(RecvVerificationRepo::new(db_connection.clone())),
            verification_req_repo: Arc::new(ReqVerificationRepo::new(db_connection.clone())),
            token_req_repo: Arc::new(RecvTokenRequirementsRepo::new(db_connection.clone())),
            authority_repo: Arc::new(VcRequestRepo::new(db_connection.clone())),
            mates_repo: Arc::new(MatesRepo::new(db_connection.clone())),
            business_mates_repo: Arc::new(BusinessMatesProviderRepo::new(db_connection)),
        }
    }
}

impl AuthProviderRepoTrait for AuthProviderRepoForSql {
    fn request_rcv(&self) -> Arc<dyn RecvRequestTrait> {
        self.request_repo.clone()
    }

    fn interaction_rcv(&self) -> Arc<dyn RecvInteractionTrait> {
        self.interaction_rcvr_repo.clone()
    }
    fn interaction_req(&self) -> Arc<dyn ReqInteractionTrait> {
        self.interaction_req_repo.clone()
    }

    fn verification_rcv(&self) -> Arc<dyn RecvVerificationTrait> {
        self.verification_rcvr_repo.clone()
    }
    fn verification_req(&self) -> Arc<dyn ReqVerificationTrait> {
        self.verification_req_repo.clone()
    }

    fn token_requirements_rcv(&self) -> Arc<dyn RecvTokenRequirementsTrait> {
        self.token_req_repo.clone()
    }

    fn vc_req(&self) -> Arc<dyn ReqVcTrait> {
        self.authority_repo.clone()
    }

    fn mates(&self) -> Arc<dyn MatesTrait> {
        self.mates_repo.clone()
    }

    fn business_mates(&self) -> Arc<dyn BusinessMatesRepoTrait> {
        self.business_mates_repo.clone()
    }
}
