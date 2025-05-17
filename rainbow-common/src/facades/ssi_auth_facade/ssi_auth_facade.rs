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

use crate::facades::ssi_auth_facade::SSIAuthFacadeTrait;
use axum::async_trait;

pub struct SSIAuthFacadeService {}
impl SSIAuthFacadeService {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl SSIAuthFacadeTrait for SSIAuthFacadeService {
    async fn authorize(&self, token: String) -> anyhow::Result<()> {
        Ok(())
    }
}
