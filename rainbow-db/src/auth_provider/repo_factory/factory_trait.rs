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

use crate::auth_provider::repo_factory::traits::{
    AuthInteractionRepoTrait, AuthRequestRepoTrait, AuthTokenRequirementsRepoTrait, AuthVerificationRepoTrait,
    BusinessMatesRepoTrait, MatesRepoTrait,
};
use sea_orm::DatabaseConnection;
use std::sync::Arc;

pub trait AuthRepoFactoryTrait: Send + Sync + Clone + 'static {
    fn request(&self) -> Arc<dyn AuthRequestRepoTrait>;
    fn interaction(&self) -> Arc<dyn AuthInteractionRepoTrait>;
    fn verification(&self) -> Arc<dyn AuthVerificationRepoTrait>;
    fn token_requirements(&self) -> Arc<dyn AuthTokenRequirementsRepoTrait>;
    fn mates(&self) -> Arc<dyn MatesRepoTrait>;
    fn business_mates(&self) -> Arc<dyn BusinessMatesRepoTrait>;
}
