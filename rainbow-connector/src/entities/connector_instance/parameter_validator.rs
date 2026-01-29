use crate::entities::common::parameters::{ParameterDefinition, ParameterType};
use serde_json::Value;
use std::collections::{HashMap, HashSet};

pub struct InstanceParameterValidator;

impl InstanceParameterValidator {
    pub fn validate(
        definitions: &[ParameterDefinition],
        values: &HashMap<String, Value>,
    ) -> Vec<String> {
        // 1. Detect unknown parameters
        let mut errors = Self::validate_unknown_parameters(definitions, values);

        // 2. Validate rules per parameter
        for def in definitions {
            if let Some(error) = Self::validate_single_parameter(def, values.get(&def.name)) {
                errors.push(error);
            }
        }

        errors
    }

    /// Detects parameters provided by user that are not in the definition list
    fn validate_unknown_parameters(
        definitions: &[ParameterDefinition],
        values: &HashMap<String, Value>,
    ) -> Vec<String> {
        let valid_names: HashSet<&String> = definitions.iter().map(|d| &d.name).collect();

        values
            .keys()
            .filter(|k| !valid_names.contains(k))
            .map(|k| format!("Unknown parameter: '{}'", k))
            .collect()
    }

    /// Orchestrates validation rules for a single parameter definition
    fn validate_single_parameter(
        def: &ParameterDefinition,
        user_value: Option<&Value>,
    ) -> Option<String> {
        // Rule A: Auto-filled parameters logic
        if def.auto_fillable.auto_filled {
            if user_value.is_some() {
                return Some(format!(
                    "Parameter '{}' is auto-filled and cannot be manually set.",
                    def.name
                ));
            }
            return None;
        }

        // Rule B: Existence logic (Required vs Optional)
        let Some(val) = user_value else {
            if def.required {
                return Some(format!("Missing required parameter: '{}'", def.name));
            }
            return None;
        };

        // Rule C: Type Validation (only if value exists)
        if !Self::check_type_compatibility(&def.param_type, val) {
            return Some(format!(
                "Type mismatch for '{}'. Expected {:?}, got: {}",
                def.name, def.param_type, val
            ));
        }

        None
    }

    /// Pure function to check if a Value matches a ParameterType
    fn check_type_compatibility(expected: &ParameterType, val: &Value) -> bool {
        match expected {
            ParameterType::String => val.is_string(),
            ParameterType::Int => val.is_i64() || val.is_u64(),
            ParameterType::Boolean => val.is_boolean(),
            ParameterType::VecString => {
                val.as_array().map_or(false, |arr| arr.iter().all(|e| e.is_string()))
            }
            ParameterType::MapStringString => {
                val.as_object().map_or(false, |obj| obj.values().all(|v| v.is_string()))
            }
            _ => true,
        }
    }
}
