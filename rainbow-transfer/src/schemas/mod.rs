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

pub static TRANSFER_REQUEST_SCHEMA: Lazy<JSONSchema> = Lazy::new(|| -> JSONSchema {
    let compiler = schema_compiler_util(include_str!(
        "../.././../rainbow-transfer/src/schemas/transfer-request.schema.json"
    ));
    JSONSchema::options().compile(&compiler).unwrap()
});

pub static TRANSFER_START_SCHEMA: Lazy<JSONSchema> = Lazy::new(|| -> JSONSchema {
    let compiler = schema_compiler_util(include_str!(
        "../.././../rainbow-transfer/src/schemas/transfer-start.schema.json"
    ));
    JSONSchema::options().compile(&compiler).unwrap()
});

pub static TRANSFER_SUSPENSION_SCHEMA: Lazy<JSONSchema> = Lazy::new(|| -> JSONSchema {
    let compiler = schema_compiler_util(include_str!(
        "../.././../rainbow-transfer/src/schemas/transfer-suspension.schema.json"
    ));
    JSONSchema::options().compile(&compiler).unwrap()
});

pub static TRANSFER_TERMINATION_SCHEMA: Lazy<JSONSchema> = Lazy::new(|| -> JSONSchema {
    let compiler = schema_compiler_util(include_str!(
        "../.././../rainbow-transfer/src/schemas/transfer-termination.schema.json"
    ));
    JSONSchema::options().compile(&compiler).unwrap()
});

pub static TRANSFER_PROCESS_SCHEMA: Lazy<JSONSchema> = Lazy::new(|| -> JSONSchema {
    let compiler = schema_compiler_util(include_str!(
        "../.././../rainbow-transfer/src/schemas/transfer-process.schema.json"
    ));
    JSONSchema::options().compile(&compiler).unwrap()
});

pub static TRANSFER_ERROR_SCHEMA: Lazy<JSONSchema> = Lazy::new(|| -> JSONSchema {
    let compiler = schema_compiler_util(include_str!(
        "../.././../rainbow-transfer/src/schemas/transfer-error.schema.json"
    ));
    JSONSchema::options().compile(&compiler).unwrap()
});

pub static TRANSFER_COMPLETION_SCHEMA: Lazy<JSONSchema> = Lazy::new(|| -> JSONSchema {
    let compiler = schema_compiler_util(include_str!(
        "../.././../rainbow-transfer/src/schemas/transfer-completion.schema.json"
    ));
    JSONSchema::options().compile(&compiler).unwrap()
});
