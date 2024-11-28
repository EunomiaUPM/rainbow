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

use rainbow_common::dcat_formats::{DctFormats, FormatAction};
use rainbow_common::protocol::transfer::DataAddress;
use tracing::debug;
use uuid::Uuid;

pub fn is_consumer_pid_valid(consumer_pid: &String) -> anyhow::Result<bool> {
    debug!("Validating consumer_pid: {}", consumer_pid);
    Ok(true)
}

pub fn is_agreement_valid(agreement_id: &String) -> anyhow::Result<bool> {
    debug!("Validating agreement_id: {}", agreement_id);
    Ok(true)
}

pub fn has_data_address_in_push(
    data_address: &Option<DataAddress>,
    format: &DctFormats,
) -> anyhow::Result<bool> {
    let format_action = &format.action;
    match format_action {
        FormatAction::Push => Ok(data_address.is_some()),
        FormatAction::Pull => Ok(data_address.is_none()),
    }
}

pub fn does_callback_exist(callback_id: Uuid) -> anyhow::Result<bool> {
    debug!("{}", callback_id.to_string());
    Ok(true)
}
