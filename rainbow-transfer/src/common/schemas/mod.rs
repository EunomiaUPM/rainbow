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

use jsonschema;
use jsonschema::Validator;
use once_cell::sync::Lazy;
use rainbow_common::schemas::schema_compiler_util;
use serde_json::Value;

pub mod validation;

// TODO make all of this in compile time...
pub static TRANSFER_REQUEST_SCHEMA: Lazy<Validator> = Lazy::new(|| -> Validator {
    let mut request_schema = schema_compiler_util(include_str!(
        "../../.././../rainbow-common/src/schemas/transfer/transfer-request-message-schema.json"
    ));
    let data_address_schema = schema_compiler_util(include_str!(
        "../../.././../rainbow-common/src/schemas/transfer/data-address-schema.json"
    ));
    let context_schema = schema_compiler_util(include_str!(
        "../../.././../rainbow-common/src/schemas/common/context-schema.json"
    ));
    let data_addres_prop = &data_address_schema["definitions"]["DataAddress"];
    let endpoint_prop = &data_address_schema["definitions"]["EndpointProperty"];
    let context_prop = &context_schema["definitions"]["ContextSchema"];
    let definitions = request_schema.get_mut("definitions").and_then(|v| v.as_object_mut()).unwrap();
    definitions.insert("DataAddress".to_string(), data_addres_prop.clone());
    definitions.insert("EndpointProperty".to_string(), endpoint_prop.clone());
    definitions.insert("ContextSchema".to_string(), context_prop.clone());
    let definitions = serde_json::to_string(&request_schema).expect("Could not serialize schema");
    let definitions = definitions.replace(
        "https://w3id.org/dspace/2025/1/transfer/data-address-schema.json",
        "#/definitions/DataAddress",
    ).replace(
        "https://w3id.org/dspace/2025/1/common/context-schema.json",
        "#/definitions/ContextSchema",
    );
    let updated_schema: Value = serde_json::from_str(&definitions).expect("Invalid updated JSON");
    let validator = jsonschema::validator_for(&updated_schema).unwrap();
    validator
});

pub static TRANSFER_START_SCHEMA: Lazy<Validator> = Lazy::new(|| -> Validator {
    let mut start_schema = schema_compiler_util(include_str!(
        "../../.././../rainbow-common/src/schemas/transfer/transfer-start-message-schema.json"
    ));
    let data_address_schema = schema_compiler_util(include_str!(
        "../../.././../rainbow-common/src/schemas/transfer/data-address-schema.json"
    ));
    let context_schema = schema_compiler_util(include_str!(
        "../../.././../rainbow-common/src/schemas/common/context-schema.json"
    ));
    let data_addres_prop = &data_address_schema["definitions"]["DataAddress"];
    let endpoint_prop = &data_address_schema["definitions"]["EndpointProperty"];
    let context_prop = &context_schema["definitions"]["ContextSchema"];
    let definitions = start_schema.get_mut("definitions").and_then(|v| v.as_object_mut()).unwrap();
    definitions.insert("DataAddress".to_string(), data_addres_prop.clone());
    definitions.insert("EndpointProperty".to_string(), endpoint_prop.clone());
    definitions.insert("ContextSchema".to_string(), context_prop.clone());
    let definitions = serde_json::to_string(&start_schema).expect("Could not serialize schema");
    let definitions = definitions.replace(
        "https://w3id.org/dspace/2025/1/transfer/data-address-schema.json",
        "#/definitions/DataAddress",
    ).replace(
        "https://w3id.org/dspace/2025/1/common/context-schema.json",
        "#/definitions/ContextSchema",
    );
    let updated_schema: Value = serde_json::from_str(&definitions).expect("Invalid updated JSON");
    let validator = jsonschema::validator_for(&updated_schema).unwrap();
    validator
});

pub static TRANSFER_SUSPENSION_SCHEMA: Lazy<Validator> = Lazy::new(|| -> Validator {
    let mut suspension_schema = schema_compiler_util(include_str!(
        "../../.././../rainbow-common/src/schemas/transfer/transfer-suspension-message-schema.json"
    ));
    let abstract_messages = schema_compiler_util(include_str!(
        "../../.././../rainbow-common/src/schemas/transfer/transfer-schema.json"
    ));
    let context_schema = schema_compiler_util(include_str!(
        "../../.././../rainbow-common/src/schemas/common/context-schema.json"
    ));
    let abstract_prop = &abstract_messages["definitions"]["AbstractTransferCodeMessage"];
    let context_prop = &context_schema["definitions"]["ContextSchema"];
    let definitions = suspension_schema.get_mut("definitions").and_then(|v| v.as_object_mut()).unwrap();
    definitions.insert("AbstractTransferCodeMessage".to_string(), abstract_prop.clone());
    definitions.insert("ContextSchema".to_string(), context_prop.clone());
    let definitions = serde_json::to_string(&suspension_schema).expect("Could not serialize schema");
    let definitions = definitions.replace(
        "https://w3id.org/dspace/2025/1/transfer/transfer-schema.json#",
        "#/",
    ).replace(
        "https://w3id.org/dspace/2025/1/common/context-schema.json",
        "#/definitions/ContextSchema",
    );
    let updated_schema: Value = serde_json::from_str(&definitions).expect("Invalid updated JSON");
    let validator = jsonschema::validator_for(&updated_schema);
    validator.unwrap()
});

pub static TRANSFER_TERMINATION_SCHEMA: Lazy<Validator> = Lazy::new(|| -> Validator {
    let mut termination_schema = schema_compiler_util(include_str!(
        "../../.././../rainbow-common/src/schemas/transfer/transfer-termination-message-schema.json"
    ));
    let abstract_messages = schema_compiler_util(include_str!(
        "../../.././../rainbow-common/src/schemas/transfer/transfer-schema.json"
    ));
    let context_schema = schema_compiler_util(include_str!(
        "../../.././../rainbow-common/src/schemas/common/context-schema.json"
    ));
    let abstract_prop = &abstract_messages["definitions"]["AbstractTransferCodeMessage"];
    let context_prop = &context_schema["definitions"]["ContextSchema"];
    let definitions = termination_schema.get_mut("definitions").and_then(|v| v.as_object_mut()).unwrap();
    definitions.insert("AbstractTransferCodeMessage".to_string(), abstract_prop.clone());
    definitions.insert("ContextSchema".to_string(), context_prop.clone());
    let definitions = serde_json::to_string(&termination_schema).expect("Could not serialize schema");
    let definitions = definitions.replace(
        "https://w3id.org/dspace/2025/1/transfer/transfer-schema.json#",
        "#/",
    ).replace(
        "https://w3id.org/dspace/2025/1/common/context-schema.json",
        "#/definitions/ContextSchema",
    );
    let updated_schema: Value = serde_json::from_str(&definitions).expect("Invalid updated JSON");
    let validator = jsonschema::validator_for(&updated_schema).unwrap();
    validator
});

pub static TRANSFER_PROCESS_SCHEMA: Lazy<Validator> = Lazy::new(|| -> Validator {
    let mut process_schema = schema_compiler_util(include_str!(
        "../../.././../rainbow-common/src/schemas/transfer/transfer-process-schema.json"
    ));
    let context_schema = schema_compiler_util(include_str!(
        "../../.././../rainbow-common/src/schemas/common/context-schema.json"
    ));
    let context_prop = &context_schema["definitions"]["ContextSchema"];
    let definitions = process_schema.get_mut("definitions").and_then(|v| v.as_object_mut()).unwrap();
    definitions.insert("ContextSchema".to_string(), context_prop.clone());
    let definitions = serde_json::to_string(&process_schema).expect("Could not serialize schema");
    let definitions = definitions.replace(
        "https://w3id.org/dspace/2025/1/common/context-schema.json",
        "#/definitions/ContextSchema",
    );
    let updated_schema: Value = serde_json::from_str(&definitions).expect("Invalid updated JSON");
    let validator = jsonschema::validator_for(&updated_schema).unwrap();
    validator
});

pub static TRANSFER_ERROR_SCHEMA: Lazy<Validator> = Lazy::new(|| -> Validator {
    let mut error_schema = schema_compiler_util(include_str!(
        "../../.././../rainbow-common/src/schemas/transfer/transfer-error-schema.json"
    ));
    let abstract_messages = schema_compiler_util(include_str!(
        "../../.././../rainbow-common/src/schemas/transfer/transfer-schema.json"
    ));
    let context_schema = schema_compiler_util(include_str!(
        "../../.././../rainbow-common/src/schemas/common/context-schema.json"
    ));
    let abstract_prop = &abstract_messages["definitions"]["AbstractTransferCodeMessage"];
    let context_prop = &context_schema["definitions"]["ContextSchema"];
    let definitions = error_schema.get_mut("definitions").and_then(|v| v.as_object_mut()).unwrap();
    definitions.insert("AbstractTransferCodeMessage".to_string(), abstract_prop.clone());
    definitions.insert("ContextSchema".to_string(), context_prop.clone());
    let definitions = serde_json::to_string(&error_schema).expect("Could not serialize schema");
    let definitions = definitions.replace(
        "https://w3id.org/dspace/2025/1/transfer/transfer-schema.json#",
        "#/",
    ).replace(
        "https://w3id.org/dspace/2025/1/common/context-schema.json",
        "#/definitions/ContextSchema",
    );
    let updated_schema: Value = serde_json::from_str(&definitions).expect("Invalid updated JSON");
    let validator = jsonschema::validator_for(&updated_schema).unwrap();
    validator
});

pub static TRANSFER_COMPLETION_SCHEMA: Lazy<Validator> = Lazy::new(|| -> Validator {
    let mut completion_schema = schema_compiler_util(include_str!(
        "../../.././../rainbow-common/src/schemas/transfer/transfer-completion-message-schema.json"
    ));
    let abstract_messages = schema_compiler_util(include_str!(
        "../../.././../rainbow-common/src/schemas/transfer/transfer-schema.json"
    ));
    let context_schema = schema_compiler_util(include_str!(
        "../../.././../rainbow-common/src/schemas/common/context-schema.json"
    ));
    let abstract_prop = &abstract_messages["definitions"]["AbstractTransferCodeMessage"];
    let context_prop = &context_schema["definitions"]["ContextSchema"];
    let definitions = completion_schema.get_mut("definitions").and_then(|v| v.as_object_mut()).unwrap();
    definitions.insert("AbstractTransferCodeMessage".to_string(), abstract_prop.clone());
    definitions.insert("ContextSchema".to_string(), context_prop.clone());
    let definitions = serde_json::to_string(&completion_schema).expect("Could not serialize schema");
    let definitions = definitions.replace(
        "https://w3id.org/dspace/2025/1/transfer/transfer-schema.json#",
        "#/",
    ).replace(
        "https://w3id.org/dspace/2025/1/common/context-schema.json",
        "#/definitions/ContextSchema",
    );
    let updated_schema: Value = serde_json::from_str(&definitions).expect("Invalid updated JSON");
    let validator = jsonschema::validator_for(&updated_schema).unwrap();
    validator
});
