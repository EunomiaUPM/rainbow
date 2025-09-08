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

use axum::async_trait;

#[async_trait]
pub trait BasicRepoTrait<T>: Send + Sync {
    async fn get_all(&self, limit: Option<u64>, offset: Option<u64>) -> anyhow::Result<Vec<T>>;
    async fn get_by_id(&self, id: &str) -> anyhow::Result<Option<T>>;
    async fn create(&self, model: T) -> anyhow::Result<T>;
    async fn delete(&self, id: &str) -> anyhow::Result<()>;
    async fn edit(&self, model: T) -> anyhow::Result<T>;
}
