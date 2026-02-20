#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    // Holds the index to find the associated local
    InMemory(usize),
    // Holds the value as-is
    Const(ContExpr),
}

/// Constant expressions
#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum ContExpr {
    Int(String),
    Bool(bool),
    Void
}
