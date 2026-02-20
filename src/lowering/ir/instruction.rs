//! IR Instructions
//! Variable-based (non-SSA) representation

use super::value::Value;
use crate::parser::ast::Span;

/// IR Instructions
#[derive(Debug)]
#[allow(dead_code)]
pub enum LValInstruct {
    /// Variable declaration with initial value
    Let {
        local_idx: usize,
        init: RValInstruct,
        span: Option<Span>,
    },
    /// Variable assignment (mutation)
    Assign {
        local_idx: usize,
        value: RValInstruct,
        span: Option<Span>,
    },
    /// Function call
    Call {
        callee: String,
        args: Vec<RValInstruct>,
        span: Option<Span>,
    },
}

#[derive(Debug)]
pub enum RValInstruct {
    // Use a value
    Use(Value, Option<Span>),
    /// Binary arithmetic operation
    Binary {
        op: BinaryOp,
        lhs: Box<RValInstruct>,
        rhs: Box<RValInstruct>,
        span: Option<Span>,
    },
    /// Unary operation
    Unary {
        op: UnaryOp,
        operand: Box<RValInstruct>,
        span: Option<Span>,
    },

    Logical {
        op: LogicalOp,
        lhs: Box<RValInstruct>,
        rhs: Box<RValInstruct>,
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
pub enum LogicalOp {
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
}
