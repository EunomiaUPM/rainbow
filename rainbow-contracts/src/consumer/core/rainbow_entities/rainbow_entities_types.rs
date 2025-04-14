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

use rainbow_common::utils::get_urn_from_string;
use rainbow_db::contracts_consumer::repo::EditContractNegotiationProcess;
use rainbow_db::contracts_consumer::repo::NewContractNegotiationProcess;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NewContractNegotiationRequest {
    #[serde(rename = "dspace:providerPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_id: Option<String>,
    #[serde(rename = "dspace:consumerPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumer_id: Option<String>,
}

impl Into<NewContractNegotiationProcess> for NewContractNegotiationRequest {
    fn into(self) -> NewContractNegotiationProcess {
        NewContractNegotiationProcess {
            provider_id: self.provider_id.map(|id| get_urn_from_string(&id).unwrap()),
            consumer_id: self.consumer_id.map(|id| get_urn_from_string(&id).unwrap()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EditContractNegotiationRequest {}

impl Into<EditContractNegotiationProcess> for EditContractNegotiationRequest {
    fn into(self) -> EditContractNegotiationProcess {
        EditContractNegotiationProcess {}
    }
}
