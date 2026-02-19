//! Values in the IR
//! Variable-based (non-SSA) representation

/// Constant expressions
#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum ContExpr {
    Int(i64),
}

/// A value reference
#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum Value {
    /// Named local variable
    Local(String),
    /// Function parameter by index
    Param(usize),
    /// Constant value
    Const(ContExpr),
}
