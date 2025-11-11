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

use crate::auth_provider::entities::mates::{Model, NewModel};
use crate::common::BasicRepoTrait;
use axum::async_trait;
use urn::Urn;

#[async_trait]
pub trait MatesRepoTrait: BasicRepoTrait<Model, NewModel> + Send + Sync {
    async fn get_me(&self) -> anyhow::Result<Option<Model>>;
    async fn get_by_token(&self, token: &str) -> anyhow::Result<Option<Model>>;
    async fn force_create(&self, mate: NewModel) -> anyhow::Result<Model>;
    async fn get_batch(&self, ids: &Vec<String>) -> anyhow::Result<Vec<Model>>;
}
