use std::fmt::{self, Display, Formatter};

use super::value::Value;
use crate::parser::ast::Span;

/// IR Instructions
#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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

impl Display for BinaryOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Add => write!(f, "+"),
            Self::Sub => write!(f, "-"),
            Self::Mul => write!(f, "*"),
            Self::Div => write!(f, "/"),
            Self::Mod => write!(f, "%"),
        }
    }
}

impl Display for UnaryOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Neg => write!(f, "-"),
            Self::Not => write!(f, "!"),
        }
    }
}

impl Display for LogicalOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Eq => write!(f, "=="),
            Self::Ne => write!(f, "!="),
            Self::Lt => write!(f, "<"),
            Self::Le => write!(f, "<="),
            Self::Gt => write!(f, ">"),
            Self::Ge => write!(f, ">="),
        }
    }
}

impl Display for RValInstruct {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Use(v, _) => write!(f, "{}", v),
            Self::Binary { op, lhs, rhs, .. } => write!(f, "{} {} {}", lhs, op, rhs),
            Self::Unary { op, operand, .. } => write!(f, "{}{}", op, operand),
            Self::Logical { op, lhs, rhs, .. } => write!(f, "{} {} {}", lhs, op, rhs),
        }
    }
}

impl Display for LValInstruct {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Let {
                local_idx, init, ..
            } => {
                write!(f, "let %{} = {}", local_idx, init)
            }
            Self::Assign {
                local_idx, value, ..
            } => {
                write!(f, "%{} = {}", local_idx, value)
            }
            Self::Call { callee, args, .. } => {
                let args_str = args
                    .iter()
                    .map(|a| a.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(f, "call {}({})", callee, args_str)
            }
        }
    }
}
