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

use super::traits::{
    BusinessMatesRepoTrait, RecvInteractionTrait, RecvRequestTrait, RecvTokenRequirementsTrait, RecvVerificationTrait,
};
use crate::auth::common::traits::{MatesTrait, ReqInteractionTrait, ReqVcTrait, ReqVerificationTrait};
use std::sync::Arc;

pub trait AuthProviderRepoTrait: Send + Sync + 'static {
    fn request_rcv(&self) -> Arc<dyn RecvRequestTrait>;
    fn interaction_rcv(&self) -> Arc<dyn RecvInteractionTrait>;
    fn interaction_req(&self) -> Arc<dyn ReqInteractionTrait>;
    fn verification_rcv(&self) -> Arc<dyn RecvVerificationTrait>;
    fn verification_req(&self) -> Arc<dyn ReqVerificationTrait>;
    fn token_requirements_rcv(&self) -> Arc<dyn RecvTokenRequirementsTrait>;
    fn vc_req(&self) -> Arc<dyn ReqVcTrait>;
    fn mates(&self) -> Arc<dyn MatesTrait>;
    fn business_mates(&self) -> Arc<dyn BusinessMatesRepoTrait>;
}
