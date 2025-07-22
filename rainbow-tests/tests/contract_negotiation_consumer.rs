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
// use crate::utils::load_env_file;
use rainbow_contracts::provider::core::rainbow_entities::rainbow_entities_types::NewParticipantRequest;

use rainbow_common::protocol::contract::contract_odrl::{ContractRequestMessageOfferTypes, OdrlAgreement, OdrlAtomicConstraint, OdrlConstraint, OdrlMessageOffer, OdrlOffer, OdrlPermission, OdrlPolicyInfo, OdrlRightOperand, OdrlTypes, Operator};
use rainbow_common::utils::{get_urn, get_urn_from_string};

use rainbow_catalog::core::rainbow_entities::rainbow_catalog_types::{NewCatalogRequest, NewDatasetRequest};
use rainbow_common::protocol::catalog::catalog_definition::Catalog;
use rainbow_common::protocol::catalog::dataset_definition::Dataset;
use rainbow_common::protocol::contract::contract_odrl::ContractRequestMessageOfferTypes::OfferMessage;
use rainbow_contracts::consumer::core::ds_protocol_rpc::ds_protocol_rpc_types::{
    SetupRequestRequest, SetupRequestResponse, SetupVerificationRequest, SetupVerificationResponse,
};
use rainbow_contracts::provider::core::ds_protocol_rpc::ds_protocol_rpc_types::{
    SetupAgreementRequest, SetupAgreementResponse, SetupFinalizationRequest, SetupFinalizationResponse,
    SetupOfferRequest, SetupOfferResponse,
};
use rainbow_db::catalog;
use rainbow_db::catalog::entities::odrl_offer;
use rainbow_db::contracts_provider::entities::participant;
use serde_json::json;
use std::process::Command;
use tracing_test::traced_test;
// #[path = "utils.rs"]
// mod utils;

#[traced_test]
#[tokio::test]
pub async fn contract_negotiation_consumer() -> anyhow::Result<()> {
    //
    // Setup servers
    //
    let cwd = "./../rainbow-core";
    // let provider_envs = load_env_file(".env.provider.template");
    let mut provider_server = Command::new("cargo")
        .current_dir(cwd)
        .args(&["run", "--", "provider", "start"])
        .spawn()
        .expect("Failed to start provider server");

    // let consumer_envs = load_env_file(".env.consumer.template");
    let mut consumer_server = Command::new("cargo")
        .current_dir(cwd)
        .args(&["run", "--", "consumer", "start"])
        .spawn()
        .expect("Failed to start consumer server");

    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

    let consumer_client = reqwest::Client::new();
    let provider_client = reqwest::Client::new();

    // -------------------------------
    // Create participants consumer and provider
    // -------------------------------
    // We create participants from scratch, there's no identity flow implemented yet
    let req = provider_client
        .post("http://localhost:1200/api/v1/participants")
        .json(&NewParticipantRequest {
            participant_id: None,
            _type: "Provider".to_string(),
            base_url: "http://127.0.0.1:1200".to_string(),
            extra_fields: Default::default(),
        })
        .send()
        .await?;
    let res = req.json::<participant::Model>().await?;
    let provider_participant = res.clone();
    let provider_participant_id = get_urn_from_string(&provider_participant.participant_id)?;
    println!("Provider participant: {:#?}", provider_participant_id);

    let req = provider_client
        .post("http://localhost:1200/api/v1/participants")
        .json(&NewParticipantRequest {
            participant_id: None,
            _type: "Consumer".to_string(),
            base_url: "http://127.0.0.1:1100".to_string(),
            extra_fields: Default::default(),
        })
        .send()
        .await?;
    let res = req.json::<participant::Model>().await?;
    let consumer_participant = res.clone();
    let consumer_participant_id = get_urn_from_string(&consumer_participant.participant_id)?;
    println!("Consumer participant: {:#?}", consumer_participant);

    // -------------------------------
    // Create Dataset and policy
    // -------------------------------
    let req = provider_client
        .post("http://localhost:1200/api/v1/catalogs")
        .json(&NewCatalogRequest {
            id: None,
            foaf_home_page: None,
            dct_conforms_to: None,
            dct_creator: None,
            dct_title: Some("my catalog...".to_string()),
        })
        .send()
        .await?;

    let res = req.json::<Catalog>().await?;
    println!("Catalog: {:#?}", res);
    let catalog_id = res.id;

    let req = provider_client
        .post(format!(
            "http://localhost:1200/api/v1/catalogs/{}/datasets",
            catalog_id
        ))
        .json(&NewDatasetRequest { id: None, dct_conforms_to: None, dct_creator: None, dct_title: None })
        .send()
        .await?;
    let res = req.json::<Dataset>().await?;
    println!("Dataset: {:#?}", res);
    let dataset_id = get_urn_from_string(&res.id)?;

    let req = provider_client
        .post(format!(
            "http://localhost:1200/api/v1/datasets/{}/policies",
            dataset_id
        ))
        .json(&OdrlPolicyInfo {
            profile: None,
            permission: Some(vec![OdrlPermission {
                action: "use".to_string(),
                constraint: Some(vec![OdrlConstraint::Atomic(OdrlAtomicConstraint {
                    right_operand: OdrlRightOperand::Str("user".to_string()),
                    left_operand: "aaa".to_string(),
                    operator: Operator::Eq,
                })]),
                duty: None,
            }]),
            obligation: None,
            prohibition: None,
        })
        .send()
        .await?;
    let res = req.json::<OdrlOffer>().await?;
    println!("Dataset policy: {:#?}", res);
    let policy_id = res.id;
    let policy_target = res.target.unwrap();

    // -------------------------------
    // Consumer inits offer with REQUESTED
    // -------------------------------
    let req = consumer_client
        .post("http://127.0.0.1:1100/api/v1/negotiations/rpc/setup-request")
        .json(&SetupRequestRequest {
            provider_address: "http://127.0.0.1:1200".to_string(),
            consumer_pid: None,
            provider_pid: None,
            odrl_offer: ContractRequestMessageOfferTypes::OfferMessage(OdrlMessageOffer {
                id: policy_id.clone(),
                profile: None,
                permission: Some(vec![OdrlPermission {
                    action: "use".to_string(),
                    constraint: None,
                    duty: None,
                }]),
                obligation: None,
                _type: OdrlTypes::Offer,
                prohibition: None,
                target: policy_target.clone(),
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
        .post("http://localhost:1200/api/v1/negotiations/rpc/setup-offer")
        .json(&SetupOfferRequest {
            consumer_participant_id: consumer_participant_id.clone(),
            consumer_pid: Option::from(consumer_pid.clone()),
            provider_pid: Option::from(provider_pid.clone()),
            odrl_offer: OdrlMessageOffer {
                id: policy_id.clone(),
                target: policy_target.clone(),
                profile: None,
                permission: Some(vec![OdrlPermission {
                    action: "superuse".to_string(),
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

    // -------------------------------
    // Consumer redoes offer with REQUEST
    // -------------------------------
    let req = consumer_client
        .post("http://127.0.0.1:1100/api/v1/negotiations/rpc/setup-request")
        .json(&SetupRequestRequest {
            provider_address: "http://127.0.0.1:1200".to_string(),
            consumer_pid: Option::from(consumer_pid.clone()),
            provider_pid: Option::from(provider_pid.clone()),
            odrl_offer: ContractRequestMessageOfferTypes::OfferMessage(OdrlMessageOffer {
                id: policy_id.clone(),
                profile: None,
                permission: Some(vec![OdrlPermission {
                    action: "superultrause".to_string(),
                    constraint: None,
                    duty: None,
                }]),
                obligation: None,
                _type: OdrlTypes::Offer,
                prohibition: None,
                target: policy_target.clone(),
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
        .post("http://127.0.0.1:1200/api/v1/negotiations/rpc/setup-agreement")
        .json(&SetupAgreementRequest {
            consumer_participant_id: consumer_participant_id.clone(),
            consumer_pid: consumer_pid.clone(),
            provider_pid: provider_pid.clone(),
        })
        .send()
        .await?;
    let res = req.json::<SetupAgreementResponse>().await?;
    println!("SetupAgreementRequest: {:#?}", res);

    // -------------------------------
    // Consumer verifies agreement with VERIFY
    // -------------------------------
    let req = consumer_client
        .post("http://127.0.0.1:1100/api/v1/negotiations/rpc/setup-verification")
        .json(&SetupVerificationRequest {
            provider_address: "http://127.0.0.1:1200".to_string(),
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
        .post("http://127.0.0.1:1200/api/v1/negotiations/rpc/setup-finalization")
        .json(&SetupFinalizationRequest {
            consumer_participant_id: consumer_participant_id.clone(),
            consumer_pid: consumer_pid.clone(),
            provider_pid: provider_pid.clone(),
        })
        .send()
        .await?;
    let res = req.json::<SetupFinalizationResponse>().await?;
    println!("SetupFinalizationResponse: {:#?}", res);

    //
    // Tear down servers
    //
    provider_server.kill().expect("Failed to kill provider server");
    consumer_server.kill().expect("Failed to kill consumer server");
    Ok(())
}
