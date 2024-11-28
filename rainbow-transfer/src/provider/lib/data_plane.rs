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

use rainbow_catalog::core::ll_api::dataservices_request_by_id;
use rainbow_catalog::protocol::dataservice_definition::DataService;
use rainbow_common::config::database::get_db_connection;
use rainbow_db::transfer_provider::entities::agreements;
use sea_orm::EntityTrait;
use uuid::Uuid;

pub async fn resolve_endpoint_from_agreement(agreement_id: Uuid) -> anyhow::Result<DataService> {
    let db_connection = get_db_connection().await;
    let agreement = agreements::Entity::find_by_id(agreement_id).one(db_connection).await?;
    if agreement.is_none() {
        return Err(anyhow::anyhow!("agreement not found"));
    }
    let agreement = agreement.unwrap();

    // TODO if is all in modules, change function
    let data_service_id = agreement.data_service_id;
    let data_service = dataservices_request_by_id(data_service_id).await?;
    if data_service.is_none() {
        return Err(anyhow::anyhow!("data service not found"));
    }

    Ok(data_service.unwrap())
}
