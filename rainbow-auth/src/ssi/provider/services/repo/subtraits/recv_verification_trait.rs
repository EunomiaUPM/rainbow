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

use crate::ssi::provider::data::entities::recv_verification::{Entity, Model, NewModel};
use axum::async_trait;
use rainbow_common::data::BasicRepoTrait;

#[async_trait]
pub trait RecvVerificationTrait: BasicRepoTrait<Entity, NewModel> + Send + Sync {
    async fn get_by_state(&self, state: &str) -> anyhow::Result<Model>;
    async fn create_from_basic(&self, model: Model) -> anyhow::Result<Model>;
}
