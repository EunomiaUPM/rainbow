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

use crate::protocol::contract::contract_negotiation_event::NegotiationEventType;
use crate::protocol::contract::contract_odrl::{ContractRequestMessageOfferTypes, OdrlAgreement};
use crate::protocol::contract::ContractNegotiationMessages;
use serde::{Deserialize, Serialize};
use urn::Urn;

pub trait DSProtocolContractNegotiationMessageTrait<'a>: Serialize + Deserialize<'a> + Clone {
    fn get_message_type(&self) -> anyhow::Result<ContractNegotiationMessages>;
    fn get_consumer_pid(&self) -> anyhow::Result<Option<&Urn>> {
        Ok(None)
    }
    fn get_provider_pid(&self) -> anyhow::Result<Option<&Urn>> {
        Ok(None)
    }
    fn get_negotiation_event_type(&self) -> anyhow::Result<Option<NegotiationEventType>> {
        Ok(None)
    }
    fn get_odrl_offer(&self) -> anyhow::Result<Option<&ContractRequestMessageOfferTypes>> {
        Ok(None)
    }
    fn get_odrl_agreement(&self) -> anyhow::Result<Option<&OdrlAgreement>> {
        Ok(None)
    }
}
