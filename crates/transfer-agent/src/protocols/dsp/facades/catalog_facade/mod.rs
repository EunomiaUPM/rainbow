/*
 * Copyright (C) 2026 - Universidad Politécnica de Madrid - UPM
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 */

use std::str::FromStr;

use catalog_agent::{DatasetDto, DistributionDto};
use common::utils::get_urn_from_string;
use connector::ConnectorInstanceDto;
use urn::Urn;
use ymir::errors::{Errors, Outcome};

pub mod local;
pub mod remote;

/// The catalog agent, as transfer needs it: datasets, distributions and the connector
/// instances it hosts, looked up by id whoever owns them (the agreement already fixed them).
#[mockall::automock]
#[async_trait::async_trait]
pub trait CatalogFacadeTrait: Send + Sync {
    async fn get_dataset(&self, dataset_id: &Urn) -> Outcome<DatasetDto>;

    async fn get_distribution_by_format(
        &self,
        dataset_id: &Urn,
        dct_format: &str,
    ) -> Outcome<DistributionDto>;

    async fn get_instance_by_distribution(
        &self,
        distribution_id: &Urn,
    ) -> Outcome<Option<ConnectorInstanceDto>>;

    /// The connector instance serving `target` in `format`: its dataset, then the distribution
    /// in that format, then the instance behind it.
    async fn get_connector_for(&self, target: &Urn, format: &str) -> Outcome<ConnectorInstanceDto> {
        let dataset = self.get_dataset(target).await?;
        let dataset_id = get_urn_from_string(&dataset.inner.id)?;
        let distribution = self.get_distribution_by_format(&dataset_id, format).await?;
        let distribution_id = Urn::from_str(distribution.inner.id.as_str())?;
        self.get_instance_by_distribution(&distribution_id)
            .await?
            .ok_or_else(|| {
                Errors::crazy(
                    format!("No connector instance found for distribution {distribution_id}"),
                    None,
                )
            })
    }
}
