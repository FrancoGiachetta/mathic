//! IR Instructions
//! Variable-based (non-SSA) representation

use super::value::{ContExpr, Value};
use crate::parser::ast::Span;

/// IR Instructions
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Instruction {
    /// Variable declaration with initial value
    Let {
        name: String,
        init: Value,
        span: Option<Span>,
    },

    /// Variable assignment (mutation)
    Assign {
        name: String,
        value: Value,
        span: Option<Span>,
    },

    /// Binary arithmetic operation
    Binary {
        dest: String,
        op: BinaryOp,
        lhs: Value,
        rhs: Value,
        span: Option<Span>,
    },

    /// Unary operation
    Unary {
        dest: String,
        op: UnaryOp,
        operand: Value,
        span: Option<Span>,
    },

    /// Comparison operation
    Compare {
        dest: String,
        pred: ComparisonOp,
        lhs: Value,
        rhs: Value,
        span: Option<Span>,
    },

    /// Function call
    Call {
        dest: String,
        callee: String,
        args: Vec<Value>,
        span: Option<Span>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum UnaryOp {
    Neg,
    Not,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum ComparisonOp {
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
}
