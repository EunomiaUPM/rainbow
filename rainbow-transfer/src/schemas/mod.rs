use jsonschema::JSONSchema;
use once_cell::sync::Lazy;
use serde_json::Value;

fn schema_compiler_util(schema_content: &str) -> Value {
    serde_json::from_str::<Value>(schema_content).unwrap()
}

pub static TRANSFER_REQUEST_SCHEMA: Lazy<JSONSchema> = Lazy::new(|| -> JSONSchema {
    let compiler = schema_compiler_util(include_str!("../.././../rainbow-transfer/src/schemas/transfer-request.schema.json"));
    JSONSchema::options().compile(&compiler).unwrap()
});

pub static TRANSFER_START_SCHEMA: Lazy<JSONSchema> = Lazy::new(|| -> JSONSchema {
    let compiler = schema_compiler_util(include_str!("../.././../rainbow-transfer/src/schemas/transfer-start.schema.json"));
    JSONSchema::options().compile(&compiler).unwrap()
});

pub static TRANSFER_SUSPENSION_SCHEMA: Lazy<JSONSchema> = Lazy::new(|| -> JSONSchema {
    let compiler = schema_compiler_util(include_str!("../.././../rainbow-transfer/src/schemas/transfer-suspension.schema.json"));
    JSONSchema::options().compile(&compiler).unwrap()
});

pub static TRANSFER_TERMINATION_SCHEMA: Lazy<JSONSchema> = Lazy::new(|| -> JSONSchema {
    let compiler = schema_compiler_util(include_str!("../.././../rainbow-transfer/src/schemas/transfer-termination.schema.json"));
    JSONSchema::options().compile(&compiler).unwrap()
});

pub static TRANSFER_PROCESS_SCHEMA: Lazy<JSONSchema> = Lazy::new(|| -> JSONSchema {
    let compiler = schema_compiler_util(include_str!("../.././../rainbow-transfer/src/schemas/transfer-process.schema.json"));
    JSONSchema::options().compile(&compiler).unwrap()
});

pub static TRANSFER_ERROR_SCHEMA: Lazy<JSONSchema> = Lazy::new(|| -> JSONSchema {
    let compiler = schema_compiler_util(include_str!("../.././../rainbow-transfer/src/schemas/transfer-error.schema.json"));
    JSONSchema::options().compile(&compiler).unwrap()
});

pub static TRANSFER_COMPLETION_SCHEMA: Lazy<JSONSchema> = Lazy::new(|| -> JSONSchema {
    let compiler = schema_compiler_util(include_str!("../.././../rainbow-transfer/src/schemas/transfer-completion.schema.json"));
    JSONSchema::options().compile(&compiler).unwrap()
});
