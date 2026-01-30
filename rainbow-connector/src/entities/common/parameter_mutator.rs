use serde_json::Value;

pub trait TemplateMutator {
    type Error: From<anyhow::Error>;
    fn enter_scope(&mut self, name: &str);
    fn exit_scope(&mut self);
    fn resolve(&self, raw: &str) -> Option<Value>;
}
