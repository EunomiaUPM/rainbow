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
use crate::utils::load_env_file;
use rainbow_contracts::provider::core::rainbow_cn_types::NewParticipantRequest;
use rainbow_contracts::provider::http::rainbow_idsa_triggers::{
    SetupAgreementRequest, SetupAgreementResponse, SetupFinalizationRequest,
    SetupFinalizationResponse, SetupOfferRequest, SetupOfferResponse,
};

use rainbow_common::protocol::contract::contract_ack::ContractAckMessage;
use rainbow_common::protocol::contract::contract_odrl::{
    OdrlAgreement, OdrlMessageOffer, OdrlOffer, OdrlPermission, OdrlTypes, OfferTypes,
};
use rainbow_common::utils::{get_urn, get_urn_from_string};
use rainbow_contracts::consumer::http::rainbow_idsa_triggers::{
    SetupAcceptanceRequest, SetupAcceptanceResponse, SetupRequestRequest, SetupRequestResponse,
    SetupVerificationRequest, SetupVerificationResponse,
};
use rainbow_dataplane::proxy::consumer_http::consumer_dataplane_router;
use rainbow_db::contracts_provider::entities::participant;
use serde_json::json;
use std::process::Command;
use tracing_test::traced_test;

#[path = "utils.rs"]
mod utils;

#[traced_test]
#[tokio::test]
pub async fn contract_negotiation_consumer() -> anyhow::Result<()> {
    ///
    /// Setup servers
    ///
    let cwd = "./../rainbow-core";
    let provider_envs = load_env_file(".env.provider.template");
    let mut provider_server = Command::new("cargo")
        .current_dir(cwd)
        .env_clear()
        .envs(&provider_envs)
        .env("TEST", "true")
        .args(&["run", "--", "provider", "start"])
        .spawn()
        .expect("Failed to start provider server");

    let consumer_envs = load_env_file(".env.consumer.template");
    let mut consumer_server = Command::new("cargo")
        .current_dir(cwd)
        .env_clear()
        .envs(&consumer_envs)
        .env("TEST", "true")
        .args(&["run", "--", "consumer", "start"])
        .spawn()
        .expect("Failed to start consumer server");

    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

    let consumer_client = reqwest::Client::new();
    let provider_client = reqwest::Client::new();

    // -------------------------------
    // Create participants consumer
    // -------------------------------
    // We create participants from scratch, there's no identity flow implemented yet
    let req = provider_client
        .post("http://localhost:1234/api/v1/participants")
        .json(&NewParticipantRequest {
            _type: "Consumer".to_string(),
            base_url: "http://127.0.0.1:1235".to_string(),
            extra_fields: Default::default(),
        })
        .send()
        .await?;
    let res = req.json::<participant::Model>().await?;
    let consumer_participant = res.clone();
    let consumer_participant_id = get_urn_from_string(&consumer_participant.participant_id)?;
    println!("Consumer participant: {:#?}", consumer_participant);

    // -------------------------------
    // Consumer inits offer with REQUESTED
    // -------------------------------
    let req = consumer_client
        .post("http://127.0.0.1:1235/api/v1/negotiations/rpc/setup-request")
        .json(&SetupRequestRequest {
            consumer_pid: None,
            provider_pid: None,
            odrl_offer: OfferTypes::Offer(OdrlOffer {
                id: get_urn(None),
                target: Option::from(get_urn(None)), // Not implemented yet before catalog cleared out...
                profile: None,
                permission: Some(vec![OdrlPermission {
                    action: "supermegause".to_string(),
                    constraint: None,
                    duty: None,
                }]),
                obligation: None,
                _type: OdrlTypes::Offer,
                prohibition: None,
            }),
        })
        .send()
        .await?;
    let res = req.json::<SetupRequestResponse>().await?;
    let consumer_pid = res.consumer_pid.clone().unwrap();
    let provider_pid = res.provider_pid.clone().unwrap();
    println!("SetupRequestResponse: {:#?}", res);

    // -------------------------------
    // Provider redoes OFFER
    // -------------------------------
    let req = provider_client
        .post("http://localhost:1234/api/v1/negotiations/rpc/setup-offer")
        .json(&SetupOfferRequest {
            consumer_participant_id: consumer_participant_id.clone(),
            consumer_pid: Option::from(consumer_pid.clone()),
            provider_pid: Option::from(provider_pid.clone()),
            odrl_offer: OdrlOffer {
                id: get_urn(None),
                target: Option::from(get_urn(None)), // Not implemented yet before catalog cleared out...
                profile: None,
                permission: Some(vec![OdrlPermission {
                    action: "use".to_string(),
                    constraint: None,
                    duty: None,
                }]),
                obligation: None,
                _type: OdrlTypes::Offer,
                prohibition: None,
            },
        })
        .send()
        .await?;
    let res = req.json::<SetupOfferResponse>().await?;
    println!("SetupOfferResponse: {:#?}", res);
    // TODO fix here, should go to OFFERED

    // -------------------------------
    // Consumer redoes offer with REQUEST
    // -------------------------------
    let req = consumer_client
        .post("http://127.0.0.1:1235/api/v1/negotiations/rpc/setup-request")
        .json(&SetupRequestRequest {
            consumer_pid: None,
            provider_pid: None,
            odrl_offer: OfferTypes::Offer(OdrlOffer {
                id: get_urn(None),
                target: Option::from(get_urn(None)), // Not implemented yet before catalog cleared out...
                profile: None,
                permission: Some(vec![OdrlPermission {
                    action: "supermegause".to_string(),
                    constraint: None,
                    duty: None,
                }]),
                obligation: None,
                _type: OdrlTypes::Offer,
                prohibition: None,
            }),
        })
        .send()
        .await?;
    let res = req.json::<SetupRequestResponse>().await?;
    println!("SetupRequestResponse: {:#?}", res);

    // -------------------------------
    // Provider agrees with AGREED and sketches agreement
    // -------------------------------
    let req = consumer_client
        .post("http://127.0.0.1:1234/api/v1/negotiations/rpc/setup-agreement")
        .json(&SetupAgreementRequest {
            consumer_participant_id: consumer_participant_id.clone(),
            consumer_pid: consumer_pid.clone(),
            provider_pid: provider_pid.clone(),
            odrl_agreement: OdrlAgreement {
                id: get_urn(None).to_string(),
                target: get_urn(None), // Not implemented yet before catalog cleared out...
                profile: None,
                permission: Some(vec![OdrlPermission {
                    action: "use".to_string(),
                    constraint: None,
                    duty: None,
                }]),
                obligation: None,
                _type: OdrlTypes::Agreement,
                prohibition: None,
                assigner: get_urn(None), // Not implemented yet, but should be participants
                assignee: get_urn(None), // Not implemented yet, but should be participants
                timestamp: None,
            },
        })
        .send()
        .await?;
    let res = req.json::<SetupAgreementResponse>().await?;
    println!("SetupAgreementRequest: {:#?}", res);

    // -------------------------------
    // Consumer verifies agreement with VERIFY
    // -------------------------------
    let req = consumer_client
        .post("http://127.0.0.1:1235/api/v1/negotiations/rpc/setup-verification")
        .json(&SetupVerificationRequest {
            consumer_pid: consumer_pid.clone(),
            provider_pid: provider_pid.clone(),
        })
        .send()
        .await?;
    let res = req.json::<SetupVerificationResponse>().await?;
    println!("SetupVerificationResponse: {:#?}", res);

    // -------------------------------
    // Provider finalizes agreement with FINALIZED
    // and creates agreement
    // -------------------------------
    let req = consumer_client
        .post("http://127.0.0.1:1234/api/v1/negotiations/rpc/setup-finalization")
        .json(&SetupFinalizationRequest {
            consumer_participant_id: consumer_participant_id.clone(),
            consumer_pid: consumer_pid.clone(),
            provider_pid: provider_pid.clone(),
        })
        .send()
        .await?;
    let res = req.json::<SetupFinalizationResponse>().await?;
    println!("SetupFinalizationResponse: {:#?}", res);

    // TODO improve strings and urns
    // TODO validation on ODRL
    // TODO persist agreement

    ///
    /// Tear down servers
    ///
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    provider_server.kill().expect("Failed to kill provider server");
    consumer_server.kill().expect("Failed to kill consumer server");
    Ok(())
}
