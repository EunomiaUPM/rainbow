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

use std::sync::Arc;

use crate::ssi::services::repo::subtraits::business_mates_trait::BusinessMatesRepoTrait;
use crate::ssi::services::repo::subtraits::recv_interaction_trait::RecvInteractionTrait;
use crate::ssi::services::repo::subtraits::recv_request_trait::RecvRequestTrait;
use crate::ssi::services::repo::subtraits::recv_verification_trait::RecvVerificationTrait;
use crate::ssi::services::repo::subtraits::req_request_trait::ReqRequestTrait;
use crate::ssi::services::repo::subtraits::{
    MatesTrait, ReqInteractionTrait, ReqVcTrait, ReqVerificationTrait, TokenRequirementsTrait
};

pub trait AuthRepoTrait: Send + Sync + 'static {
    fn request_req(&self) -> Arc<dyn ReqRequestTrait>;
    fn request_rcv(&self) -> Arc<dyn RecvRequestTrait>;
    fn interaction_req(&self) -> Arc<dyn ReqInteractionTrait>;
    fn interaction_rcv(&self) -> Arc<dyn RecvInteractionTrait>;
    fn verification_req(&self) -> Arc<dyn ReqVerificationTrait>;
    fn verification_rcv(&self) -> Arc<dyn RecvVerificationTrait>;
    fn token_requirements(&self) -> Arc<dyn TokenRequirementsTrait>;
    fn mates(&self) -> Arc<dyn MatesTrait>;
    fn business_mates(&self) -> Arc<dyn BusinessMatesRepoTrait>;
    fn vc_req(&self) -> Arc<dyn ReqVcTrait>;
}
