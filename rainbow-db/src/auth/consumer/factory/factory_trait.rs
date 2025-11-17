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

use super::traits::ReqRequestTrait;
use crate::auth::common::traits::{
    MatesTrait, ReqInteractionTrait, ReqVcTrait, ReqVerificationTrait, TokenRequirementsTrait,
};
use std::sync::Arc;

pub trait AuthConsumerRepoTrait: Send + Sync + 'static {
    fn request_req(&self) -> Arc<dyn ReqRequestTrait>;
    fn interaction_req(&self) -> Arc<dyn ReqInteractionTrait>;
    fn verification_req(&self) -> Arc<dyn ReqVerificationTrait>;
    fn token_requirements(&self) -> Arc<dyn TokenRequirementsTrait>;
    fn mates(&self) -> Arc<dyn MatesTrait>;
    fn vc_req(&self) -> Arc<dyn ReqVcTrait>;
}
