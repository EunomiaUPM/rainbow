
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ExpectedType {
    AnyString,
    StrictInt,
    StrictBool,
    StrictVec,
    StrictMap,
}

pub trait ParameterVisitor {
    type Error;
    fn enter_scope(&mut self, name: &str);
    fn exit_scope(&mut self);
    fn scan_template_candidate(&mut self, value: &str, expected: ExpectedType);
}
