use std::fs;

use jsonschema::JSONSchema;
use once_cell::sync::Lazy;
use serde_json::Value;

static SCHEMAS_ROUTE: &str = "./../rainbow-transfer/src/schemas";

fn schema_compiler_util(path: &str) -> Value {
    let file_url = format!("{}{}", SCHEMAS_ROUTE, path);
    let json_raw = fs::read_to_string(file_url).unwrap();
    let value = serde_json::from_str::<Value>(&json_raw).unwrap();
    value
}

pub static TRANSFER_REQUEST_SCHEMA: Lazy<JSONSchema> = Lazy::new(|| -> JSONSchema {
    let compiler = schema_compiler_util("/transfer-request.schema.json");
    JSONSchema::options().compile(&compiler).unwrap()
});

pub static TRANSFER_START_SCHEMA: Lazy<JSONSchema> = Lazy::new(|| -> JSONSchema {
    let compiler = schema_compiler_util("/transfer-start.schema.json");
    JSONSchema::options().compile(&compiler).unwrap()
});

pub static TRANSFER_SUSPENSION_SCHEMA: Lazy<JSONSchema> = Lazy::new(|| -> JSONSchema {
    let compiler = schema_compiler_util("/transfer-suspension.schema.json");
    JSONSchema::options().compile(&compiler).unwrap()
});

pub static TRANSFER_TERMINATION_SCHEMA: Lazy<JSONSchema> = Lazy::new(|| -> JSONSchema {
    let compiler = schema_compiler_util("/transfer-termination.schema.json");
    JSONSchema::options().compile(&compiler).unwrap()
});

pub static TRANSFER_PROCESS_SCHEMA: Lazy<JSONSchema> = Lazy::new(|| -> JSONSchema {
    let compiler = schema_compiler_util("/transfer-process.schema.json");
    JSONSchema::options().compile(&compiler).unwrap()
});

pub static TRANSFER_ERROR_SCHEMA: Lazy<JSONSchema> = Lazy::new(|| -> JSONSchema {
    let compiler = schema_compiler_util("/transfer-error.schema.json");
    JSONSchema::options().compile(&compiler).unwrap()
});

pub static TRANSFER_COMPLETION_SCHEMA: Lazy<JSONSchema> = Lazy::new(|| -> JSONSchema {
    let compiler = schema_compiler_util("/transfer-completion.schema.json");
    JSONSchema::options().compile(&compiler).unwrap()
});
