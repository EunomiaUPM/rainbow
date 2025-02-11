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

use jsonschema::JSONSchema;
use once_cell::sync::Lazy;
use serde_json::Value;

fn schema_compiler_util(schema_content: &str) -> Value {
    serde_json::from_str::<Value>(schema_content).unwrap()
}

pub static CONTRACT_SCHEMA_SCHEMA: Lazy<JSONSchema> = Lazy::new(|| -> JSONSchema {
    let compiler = schema_compiler_util(include_str!(
        "../.././../rainbow-contracts/src/schemas/contract-schema.schema.json"
    ));
    JSONSchema::options().compile(&compiler).unwrap()
});

pub static CONTRACT_REQUEST_MESSAGE_SCHEMA: Lazy<JSONSchema> = Lazy::new(|| -> JSONSchema {
    let compiler = schema_compiler_util(include_str!(
        "../.././../rainbow-contracts/src/schemas/contract-request-message.schema.json"
    ));
    JSONSchema::options().compile(&compiler).unwrap()
});

pub static CONTRACT_OFFER_MESSAGE_SCHEMA: Lazy<JSONSchema> = Lazy::new(|| -> JSONSchema {
    let compiler = schema_compiler_util(include_str!(
        "../.././../rainbow-contracts/src/schemas/contract-offer-message.schema.json"
    ));
    JSONSchema::options().compile(&compiler).unwrap()
});

pub static CONTRACT_NEGOTIATION_TERMINATION_MESSAGE_SCHEMA: Lazy<JSONSchema> = Lazy::new(|| -> JSONSchema {
    let compiler = schema_compiler_util(include_str!(
        "../.././../rainbow-contracts/src/schemas/contract-negotiation-termination-message.schema.json"
    ));
    JSONSchema::options().compile(&compiler).unwrap()
});

pub static CONTRACT_NEGOTIATION_EVENT_MESSAGE_SCHEMA: Lazy<JSONSchema> = Lazy::new(|| -> JSONSchema {
    let compiler = schema_compiler_util(include_str!(
        "../.././../rainbow-contracts/src/schemas/contract-negotiation-event-message.schema.json"
    ));
    JSONSchema::options().compile(&compiler).unwrap()
});

pub static CONTRACT_NEGOTIATION_ERROR_SCHEMA: Lazy<JSONSchema> = Lazy::new(|| -> JSONSchema {
    let compiler = schema_compiler_util(include_str!(
        "../.././../rainbow-contracts/src/schemas/contract-negotiation-error.schema.json"
    ));
    JSONSchema::options().compile(&compiler).unwrap()
});

pub static CONTRACT_NEGOTIATION_SCHEMA: Lazy<JSONSchema> = Lazy::new(|| -> JSONSchema {
    let compiler = schema_compiler_util(include_str!(
        "../.././../rainbow-contracts/src/schemas/contract-negotiation.schema.json"
    ));
    JSONSchema::options().compile(&compiler).unwrap()
});

pub static CONTRACT_AGREEMENT_VERIFICATION_MESSAGE_SCHEMA: Lazy<JSONSchema> = Lazy::new(|| -> JSONSchema {
    let compiler = schema_compiler_util(include_str!(
        "../.././../rainbow-contracts/src/schemas/contract-agreement-verification-message.schema.json"
    ));
    JSONSchema::options().compile(&compiler).unwrap()
});

pub static CONTRACT_AGREEMENT_MESSAGE_SCHEMA: Lazy<JSONSchema> = Lazy::new(|| -> JSONSchema {
    let compiler = schema_compiler_util(include_str!(
        "../.././../rainbow-contracts/src/schemas/contract-agreement-message.schema.json"
    ));
    JSONSchema::options().compile(&compiler).unwrap()
});
