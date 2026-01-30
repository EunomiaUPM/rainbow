use crate::entities::common::parameter_mutator::TemplateMutator;
use regex::Regex;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::OnceLock;

fn template_regex() -> &'static Regex {
    static REGEX: OnceLock<Regex> = OnceLock::new();
    REGEX.get_or_init(|| Regex::new(r"\{\{\s*__(.*?)__\s*\}\}").expect("Invalid Regex"))
}

pub struct TemplateResolver<'a> {
    values: &'a HashMap<String, Value>,
    context_stack: Vec<String>,
}

impl<'a> TemplateResolver<'a> {
    pub fn new(values: &'a HashMap<String, Value>) -> Self {
        Self { values, context_stack: vec![] }
    }

    fn try_exact_replacement(&self, re: &Regex, raw: &str) -> Option<Value> {
        let caps = re.captures(raw)?;
        let full_match = caps.get(0)?.as_str();
        if full_match != raw {
            return None;
        }
        let key = caps.get(1)?.as_str();
        self.values.get(key).cloned()
    }
    fn try_string_interpolation(&self, re: &Regex, raw: &str) -> Option<Value> {
        if !re.is_match(raw) {
            return None;
        }
        let mut new_string = raw.to_string();
        for caps in re.captures_iter(raw) {
            let full_match = &caps[0];
            let key = &caps[1];

            if let Some(val) = self.values.get(key) {
                let replacement_str = self.value_to_string(val);
                new_string = new_string.replace(full_match, &replacement_str);
            }
        }
        Some(Value::String(new_string))
    }

    fn value_to_string(&self, val: &Value) -> String {
        match val {
            Value::String(s) => s.clone(),
            _ => val.to_string(),
        }
    }
}

impl<'a> TemplateMutator for TemplateResolver<'a> {
    type Error = anyhow::Error;

    fn enter_scope(&mut self, name: &str) {
        self.context_stack.push(name.to_string());
    }

    fn exit_scope(&mut self) {
        self.context_stack.pop();
    }

    fn resolve(&self, raw: &str) -> Option<Value> {
        let re = template_regex();
        if let Some(val) = self.try_exact_replacement(re, raw) {
            return Some(val);
        }
        self.try_string_interpolation(re, raw)
    }
}
