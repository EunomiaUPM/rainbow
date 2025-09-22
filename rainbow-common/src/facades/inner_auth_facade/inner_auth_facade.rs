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

use axum::async_trait;
use crate::facades::inner_auth_facade::InnerAuthFacadeTrait;

pub struct InnerAuthFacadeService {}
impl InnerAuthFacadeService {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl InnerAuthFacadeTrait for InnerAuthFacadeService {
    async fn authenticate(&self) -> anyhow::Result<()> {
        Ok(())
    }

    async fn authorize(&self) -> anyhow::Result<()> {
        Ok(())
    }
}