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
use rainbow_common::err::transfer_err::TransferErrorType::{PidSchemeError, PidUuidError};
use serde_json::Value;
use uuid::Uuid;

async fn _check_urn(pid: Option<&str>) -> anyhow::Result<()> {
    if pid.is_some() {
        let scheme = pid.unwrap().contains("urn:uuid:");
        let uuid_value = Uuid::parse_str(&pid.unwrap().replace("urn:uuid:", ""));
        if uuid_value.is_err() {
            bail!(PidUuidError)
        }
        if scheme == false {
            bail!(PidSchemeError)
        }
    }
    Ok(())
}
pub async fn pids_as_urn_validation(json_value: Value) -> anyhow::Result<()> {
    let provider_pid = json_value.get("providerPid").and_then(|v| v.as_str());
    let consumer_pid = json_value.get("consumerPid").and_then(|v| v.as_str());
    let agreement_pid = json_value.get("agreementId").and_then(|v| v.as_str());

    _check_urn(provider_pid).await?;
    _check_urn(consumer_pid).await?;
    _check_urn(agreement_pid).await?;

    Ok(())
}
