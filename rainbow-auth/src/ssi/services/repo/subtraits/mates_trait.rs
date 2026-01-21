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

use axum::async_trait;
use rainbow_common::data::BasicRepoTrait;
use urn::Urn;

use crate::ssi::data::entities::mates::{Entity, Model, NewModel};

#[async_trait]
pub trait MatesTrait: BasicRepoTrait<Entity, NewModel> + Send + Sync {
    async fn get_me(&self) -> anyhow::Result<Model>;
    async fn get_by_token(&self, token: &str) -> anyhow::Result<Model>;
    async fn force_create(&self, mate: NewModel) -> anyhow::Result<Model>;
    async fn get_batch(&self, ids: &Vec<Urn>) -> anyhow::Result<Vec<Model>>;
}
