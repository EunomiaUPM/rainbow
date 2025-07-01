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

use crate::provider::core::agreement_resolver_facade::AgreementResolverFacadeTrait;
use axum::async_trait;
use rainbow_common::protocol::contract::contract_odrl::{OdrlAgreement, OdrlTypes};
use rainbow_common::utils::get_urn;
use urn::Urn;

pub struct AgreementResolverFacadeService {}

impl AgreementResolverFacadeService {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl AgreementResolverFacadeTrait for AgreementResolverFacadeService {
    async fn resolve_agreement(&self, agreement_id: Urn) -> anyhow::Result<OdrlAgreement> {
        Ok(OdrlAgreement {
            id: agreement_id,
            profile: None,
            permission: None,
            obligation: None,
            _type: OdrlTypes::Agreement,
            target: get_urn(None),
            assigner: "".to_string(),
            assignee: "".to_string(),
            timestamp: None,
            prohibition: None,
        })
    }
}
