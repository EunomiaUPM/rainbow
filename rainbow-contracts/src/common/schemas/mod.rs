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

use jsonschema::Validator;
use once_cell::sync::Lazy;
use rainbow_common::schemas::schema_compiler_util;
use serde_json::Value;

pub mod validation;

pub static CONTRACT_REQUEST_MESSAGE_SCHEMA: Lazy<Validator> = Lazy::new(|| -> Validator {
    let context_schema = schema_compiler_util(include_str!(
        "../../.././../rainbow-common/src/schemas/common/context-schema.json"
    ));
    let contract_schema = schema_compiler_util(include_str!(
        "../../.././../rainbow-common/src/schemas/negotiation/contract-schema.json"
    ));
    let mut contract_request_schema = schema_compiler_util(include_str!(
        "../../.././../rainbow-common/src/schemas/negotiation/contract-request-message-schema.json"
    ));
    let context_prop = &context_schema["definitions"]["ContextSchema"];
    let message_offer_prop = &contract_schema["definitions"]["MessageOffer"];
    let policy_class_prop = &contract_schema["definitions"]["PolicyClass"];
    let duty_prop = &contract_schema["definitions"]["Duty"];
    let permission_prop = &contract_schema["definitions"]["Permission"];
    let action_prop = &contract_schema["definitions"]["Action"];
    let constraint_prop = &contract_schema["definitions"]["Constraint"];
    let logical_constraint_prop = &contract_schema["definitions"]["LogicalConstraint"];
    let atomic_constraint_prop = &contract_schema["definitions"]["AtomicConstraint"];
    let left_operand_prop = &contract_schema["definitions"]["LeftOperand"];
    let operand_prop = &contract_schema["definitions"]["Operator"];
    let right_operand_prop = &contract_schema["definitions"]["RightOperand"];

    let definitions = contract_request_schema.get_mut("definitions").and_then(|v| v.as_object_mut()).unwrap();
    definitions.insert("ContextSchema".to_string(), context_prop.clone());
    definitions.insert("PolicyClass".to_string(), policy_class_prop.clone());
    definitions.insert("MessageOffer".to_string(), message_offer_prop.clone());
    definitions.insert("Duty".to_string(), duty_prop.clone());
    definitions.insert("Permission".to_string(), permission_prop.clone());
    definitions.insert("Action".to_string(), action_prop.clone());
    definitions.insert("Constraint".to_string(), constraint_prop.clone());
    definitions.insert("LogicalConstraint".to_string(), logical_constraint_prop.clone());
    definitions.insert("AtomicConstraint".to_string(), atomic_constraint_prop.clone());
    definitions.insert("LeftOperand".to_string(), left_operand_prop.clone());
    definitions.insert("Operator".to_string(), operand_prop.clone());
    definitions.insert("RightOperand".to_string(), right_operand_prop.clone());

    let definitions = serde_json::to_string(&contract_request_schema).expect("Could not serialize schema");
    let definitions = definitions.replace(
        "https://w3id.org/dspace/2025/1/negotiation/contract-schema.json#/definitions/MessageOffer",
        "#/definitions/MessageOffer",
    ).replace(
        "https://w3id.org/dspace/2025/1/common/context-schema.json",
        "#/definitions/ContextSchema",
    );
    let updated_schema: Value = serde_json::from_str(&definitions).expect("Invalid updated JSON");
    let validator = jsonschema::validator_for(&updated_schema).unwrap();
    validator
});

pub static CONTRACT_OFFER_MESSAGE_SCHEMA: Lazy<Validator> = Lazy::new(|| -> Validator {
    let context_schema = schema_compiler_util(include_str!(
        "../../.././../rainbow-common/src/schemas/common/context-schema.json"
    ));
    let contract_schema = schema_compiler_util(include_str!(
        "../../.././../rainbow-common/src/schemas/negotiation/contract-schema.json"
    ));
    let mut contract_offer_schema = schema_compiler_util(include_str!(
        "../../.././../rainbow-common/src/schemas/negotiation/contract-offer-message-schema.json"
    ));
    let context_prop = &context_schema["definitions"]["ContextSchema"];
    let message_offer_prop = &contract_schema["definitions"]["MessageOffer"];
    let policy_class_prop = &contract_schema["definitions"]["PolicyClass"];
    let duty_prop = &contract_schema["definitions"]["Duty"];
    let permission_prop = &contract_schema["definitions"]["Permission"];
    let action_prop = &contract_schema["definitions"]["Action"];
    let constraint_prop = &contract_schema["definitions"]["Constraint"];
    let logical_constraint_prop = &contract_schema["definitions"]["LogicalConstraint"];
    let atomic_constraint_prop = &contract_schema["definitions"]["AtomicConstraint"];
    let left_operand_prop = &contract_schema["definitions"]["LeftOperand"];
    let operand_prop = &contract_schema["definitions"]["Operator"];
    let right_operand_prop = &contract_schema["definitions"]["RightOperand"];

    let definitions = contract_offer_schema.get_mut("definitions").and_then(|v| v.as_object_mut()).unwrap();
    definitions.insert("ContextSchema".to_string(), context_prop.clone());
    definitions.insert("PolicyClass".to_string(), policy_class_prop.clone());
    definitions.insert("MessageOffer".to_string(), message_offer_prop.clone());
    definitions.insert("Duty".to_string(), duty_prop.clone());
    definitions.insert("Permission".to_string(), permission_prop.clone());
    definitions.insert("Action".to_string(), action_prop.clone());
    definitions.insert("Constraint".to_string(), constraint_prop.clone());
    definitions.insert("LogicalConstraint".to_string(), logical_constraint_prop.clone());
    definitions.insert("AtomicConstraint".to_string(), atomic_constraint_prop.clone());
    definitions.insert("LeftOperand".to_string(), left_operand_prop.clone());
    definitions.insert("Operator".to_string(), operand_prop.clone());
    definitions.insert("RightOperand".to_string(), right_operand_prop.clone());
    let definitions = serde_json::to_string(&contract_offer_schema).expect("Could not serialize schema");
    let definitions = definitions.replace(
        "https://w3id.org/dspace/2025/1/negotiation/contract-schema.json#/definitions/MessageOffer",
        "#/definitions/MessageOffer",
    ).replace(
        "https://w3id.org/dspace/2025/1/common/context-schema.json",
        "#/definitions/ContextSchema",
    );
    let updated_schema: Value = serde_json::from_str(&definitions).expect("Invalid updated JSON");
    let validator = jsonschema::validator_for(&updated_schema).unwrap();
    validator
});

pub static CONTRACT_NEGOTIATION_EVENT_MESSAGE_SCHEMA: Lazy<Validator> = Lazy::new(|| -> Validator {
    let context_schema = schema_compiler_util(include_str!(
        "../../.././../rainbow-common/src/schemas/common/context-schema.json"
    ));
    let mut contract_negotiation_schema = schema_compiler_util(include_str!(
        "../../.././../rainbow-common/src/schemas/negotiation/contract-negotiation-event-message-schema.json"
    ));
    let context_prop = &context_schema["definitions"]["ContextSchema"];
    let definitions = contract_negotiation_schema.get_mut("definitions").and_then(|v| v.as_object_mut()).unwrap();
    definitions.insert("ContextSchema".to_string(), context_prop.clone());
    let definitions = serde_json::to_string(&contract_negotiation_schema).expect("Could not serialize schema");
    let definitions = definitions.replace(
        "https://w3id.org/dspace/2025/1/negotiation/contract-schema.json#/definitions/MessageOffer",
        "#/definitions/MessageOffer",
    ).replace(
        "https://w3id.org/dspace/2025/1/common/context-schema.json",
        "#/definitions/ContextSchema",
    );
    let updated_schema: Value = serde_json::from_str(&definitions).expect("Invalid updated JSON");
    let validator = jsonschema::validator_for(&updated_schema).unwrap();
    validator
});

pub static CONTRACT_AGREEMENT_MESSAGE_SCHEMA: Lazy<Validator> = Lazy::new(|| -> Validator {
    let context_schema = schema_compiler_util(include_str!(
        "../../.././../rainbow-common/src/schemas/common/context-schema.json"
    ));
    let contract_schema = schema_compiler_util(include_str!(
        "../../.././../rainbow-common/src/schemas/negotiation/contract-schema.json"
    ));
    let mut contract_agreement_schema = schema_compiler_util(include_str!(
        "../../.././../rainbow-common/src/schemas/negotiation/contract-agreement-message-schema.json"
    ));
    // let context_prop = &context_schema["definitions"]["ContextSchema"];
    // let agreement_prop = &context_schema["definitions"]["Agreement"];
    // let policy_class_prop = &context_schema["definitions"]["PolicyClass"];
    // let definitions = contract_agreement_schema.get_mut("definitions").and_then(|v| v.as_object_mut()).unwrap();
    // definitions.insert("ContextSchema".to_string(), context_prop.clone());
    // definitions.insert("PolicyClass".to_string(), policy_class_prop.clone());
    // definitions.insert("Agreement".to_string(), agreement_prop.clone());


    let context_prop = &context_schema["definitions"]["ContextSchema"];
    let message_offer_prop = &contract_schema["definitions"]["Agreement"];
    let policy_class_prop = &contract_schema["definitions"]["PolicyClass"];
    let duty_prop = &contract_schema["definitions"]["Duty"];
    let permission_prop = &contract_schema["definitions"]["Permission"];
    let action_prop = &contract_schema["definitions"]["Action"];
    let constraint_prop = &contract_schema["definitions"]["Constraint"];
    let logical_constraint_prop = &contract_schema["definitions"]["LogicalConstraint"];
    let atomic_constraint_prop = &contract_schema["definitions"]["AtomicConstraint"];
    let left_operand_prop = &contract_schema["definitions"]["LeftOperand"];
    let operand_prop = &contract_schema["definitions"]["Operator"];
    let right_operand_prop = &contract_schema["definitions"]["RightOperand"];

    let definitions = contract_agreement_schema.get_mut("definitions").and_then(|v| v.as_object_mut()).unwrap();
    definitions.insert("ContextSchema".to_string(), context_prop.clone());
    definitions.insert("PolicyClass".to_string(), policy_class_prop.clone());
    definitions.insert("Agreement".to_string(), message_offer_prop.clone());
    definitions.insert("Duty".to_string(), duty_prop.clone());
    definitions.insert("Permission".to_string(), permission_prop.clone());
    definitions.insert("Action".to_string(), action_prop.clone());
    definitions.insert("Constraint".to_string(), constraint_prop.clone());
    definitions.insert("LogicalConstraint".to_string(), logical_constraint_prop.clone());
    definitions.insert("AtomicConstraint".to_string(), atomic_constraint_prop.clone());
    definitions.insert("LeftOperand".to_string(), left_operand_prop.clone());
    definitions.insert("Operator".to_string(), operand_prop.clone());
    definitions.insert("RightOperand".to_string(), right_operand_prop.clone());

    let definitions = serde_json::to_string(&contract_agreement_schema).expect("Could not serialize schema");
    let definitions = definitions.replace(
        "https://w3id.org/dspace/2025/1/negotiation/contract-schema.json#/definitions/Agreement",
        "#/definitions/Agreement",
    ).replace(
        "https://w3id.org/dspace/2025/1/common/context-schema.json",
        "#/definitions/ContextSchema",
    );
    let updated_schema: Value = serde_json::from_str(&definitions).expect("Invalid updated JSON");
    let validator = jsonschema::validator_for(&updated_schema).unwrap();
    validator
});

pub static CONTRACT_AGREEMENT_VERIFICATION_MESSAGE_SCHEMA: Lazy<Validator> = Lazy::new(|| -> Validator {
    let context_schema = schema_compiler_util(include_str!(
        "../../.././../rainbow-common/src/schemas/common/context-schema.json"
    ));
    let mut contract_negotiation_schema = schema_compiler_util(include_str!(
        "../../.././../rainbow-common/src/schemas/negotiation/contract-agreement-verification-message-schema.json"
    ));
    let context_prop = &context_schema["definitions"]["ContextSchema"];
    let definitions = contract_negotiation_schema.get_mut("definitions").and_then(|v| v.as_object_mut()).unwrap();
    definitions.insert("ContextSchema".to_string(), context_prop.clone());
    let definitions = serde_json::to_string(&contract_negotiation_schema).expect("Could not serialize schema");
    let definitions = definitions.replace(
        "https://w3id.org/dspace/2025/1/negotiation/contract-schema.json#/definitions/MessageOffer",
        "#/definitions/MessageOffer",
    ).replace(
        "https://w3id.org/dspace/2025/1/common/context-schema.json",
        "#/definitions/ContextSchema",
    );
    let updated_schema: Value = serde_json::from_str(&definitions).expect("Invalid updated JSON");
    let validator = jsonschema::validator_for(&updated_schema).unwrap();
    validator
});

pub static CONTRACT_TERMINATION_MESSAGE_SCHEMA: Lazy<Validator> = Lazy::new(|| -> Validator {
    let context_schema = schema_compiler_util(include_str!(
        "../../.././../rainbow-common/src/schemas/common/context-schema.json"
    ));
    let mut contract_negotiation_schema = schema_compiler_util(include_str!(
        "../../.././../rainbow-common/src/schemas/negotiation/contract-negotiation-termination-message-schema.json"
    ));
    let context_prop = &context_schema["definitions"]["ContextSchema"];
    let definitions = contract_negotiation_schema.get_mut("definitions").and_then(|v| v.as_object_mut()).unwrap();
    definitions.insert("ContextSchema".to_string(), context_prop.clone());
    let definitions = serde_json::to_string(&contract_negotiation_schema).expect("Could not serialize schema");
    let definitions = definitions.replace(
        "https://w3id.org/dspace/2025/1/negotiation/contract-schema.json#/definitions/MessageOffer",
        "#/definitions/MessageOffer",
    ).replace(
        "https://w3id.org/dspace/2025/1/common/context-schema.json",
        "#/definitions/ContextSchema",
    );
    let updated_schema: Value = serde_json::from_str(&definitions).expect("Invalid updated JSON");
    let validator = jsonschema::validator_for(&updated_schema).unwrap();
    validator
});