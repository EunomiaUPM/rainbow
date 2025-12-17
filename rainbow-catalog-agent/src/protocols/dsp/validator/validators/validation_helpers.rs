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

use crate::protocols::dsp::validator::traits::validation_helpers::ValidationHelpers;
use rainbow_common::protocol::transfer::TransferRoles;
use urn::Urn;

pub struct ValidationHelperService {}
impl ValidationHelperService {
    pub fn new() -> Self {
        Self {}
    }
}
#[async_trait::async_trait]
impl ValidationHelpers for ValidationHelperService {
    async fn parse_urn(&self, uri_id: &String) -> anyhow::Result<Urn> {
        todo!()
    }

    async fn parse_identifier_into_role(&self, identifier: &str) -> anyhow::Result<TransferRoles> {
        todo!()
    }

    async fn parse_role_into_identifier(&self, role: &TransferRoles) -> anyhow::Result<&str> {
        todo!()
    }
}
