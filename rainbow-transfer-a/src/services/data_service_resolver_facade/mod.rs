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

use rainbow_common::dcat_formats::DctFormats;
use rainbow_common::protocol::catalog::dataservice_definition::DataService;
use urn::Urn;

pub mod data_service_resolver_datahub_facade;
pub mod data_service_resolver_facade;

#[mockall::automock]
#[async_trait::async_trait]
pub trait DataServiceFacadeTrait: Send + Sync {
    async fn resolve_data_service_by_agreement_id(
        &self,
        agreement_id: Urn,
        formats: Option<DctFormats>,
    ) -> anyhow::Result<DataService>;
}
