/*
 * Copyright (C) 2026 - Universidad Politécnica de Madrid - UPM
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 */

//! JSON Schema as a rule, so published schemas (DSP 2025-1) feed the `Shape` stage.
//!
//! Validation is offline: every `$ref` must point at a schema passed in with
//! [`SchemaRule::with_resources`]; nothing is fetched over the network.

use crate::validation::rule::Rule;
use crate::validation::validator::Validator;
use crate::validation::violation::{codes, Path, Violation, Violations};
use serde_json::Value;
use ymir::errors::{BadFormat, Errors, Outcome};

/// A compiled JSON Schema; each schema error becomes a `MALFORMED` violation at its path.
pub struct SchemaRule {
    validator: jsonschema::Validator,
}

impl SchemaRule {
    /// Schema without external references.
    pub fn new(schema: &str) -> Outcome<Self> {
        Self::with_resources(schema, &[])
    }

    /// Schema whose `$ref`s resolve against `(uri, schema)` pairs given here.
    pub fn with_resources(schema: &str, resources: &[(&str, &str)]) -> Outcome<Self> {
        let root = Self::parse(schema)?;
        let mut builder = jsonschema::Registry::new();
        for (uri, contents) in resources {
            builder = builder
                .add(*uri, Self::parse(contents)?)
                .map_err(|e| Self::invalid(format!("resource {uri}: {e}")))?;
        }
        let registry = builder
            .prepare()
            .map_err(|e| Self::invalid(format!("schema registry: {e}")))?;
        let validator = jsonschema::options()
            .with_registry(&registry)
            .build(&root)
            .map_err(|e| Self::invalid(e.to_string()))?;
        Ok(Self { validator })
    }

    fn parse(json: &str) -> Outcome<Value> {
        serde_json::from_str(json).map_err(|e| Self::invalid(e.to_string()))
    }

    fn invalid(reason: String) -> Errors {
        Errors::format(
            BadFormat::Unknown,
            format!("invalid JSON Schema: {reason}"),
            None,
        )
    }
}

impl Rule<Value> for SchemaRule {
    fn check(&self, subject: &Value) -> Result<(), Violations> {
        self.validator
            .iter_errors(subject)
            .map(|e| {
                let at = e.instance_path().as_str();
                let path = if at.is_empty() {
                    "$".to_string()
                } else {
                    at.to_string()
                };
                Violation::new(Path::from(path), codes::MALFORMED, e.to_string())
            })
            .collect::<Violations>()
            .into_result()
    }
}

impl<S: 'static> Validator<S> {
    /// Check a JSON part of the subject against a schema in the current stage.
    pub fn json(
        self,
        extractor: impl Fn(&S) -> &Value + Send + Sync + 'static,
        schema: SchemaRule,
    ) -> Self {
        self.rule(move |s: &S| schema.check(extractor(s)))
    }
}
