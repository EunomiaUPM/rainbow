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
use anyhow::bail;
use rainbow_catalog::core::ll_api::dataservices_request_by_id;
use rainbow_catalog::protocol::dataservice_definition::DataService;
use rainbow_common::config::database::get_db_connection;
use rainbow_db::transfer_provider::repo::TRANSFER_PROVIDER_REPO;
use uuid::Uuid;

pub async fn resolve_endpoint_from_agreement(agreement_id: Uuid) -> anyhow::Result<DataService> {
    let db_connection = get_db_connection().await;

    let agreement = match TRANSFER_PROVIDER_REPO.get_agreement_by_id(agreement_id).await? {
        None => bail!("agreement not found"),
        Some(agreement) => agreement,
    };

    // TODO if is all in modules, change function
    let data_service_id = agreement.data_service_id;
    let data_service = match dataservices_request_by_id(data_service_id).await? {
        None => bail!("data service not found"),
        Some(data_service) => data_service,
    };

    Ok(data_service)
}
