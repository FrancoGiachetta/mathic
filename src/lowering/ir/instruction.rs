use std::fmt::{self, Display, Formatter};

use super::value::Value;
use crate::parser::ast::{
    Span,
    expression::{ArithOp, BinaryOp, CmpOp, LogicalOp, UnaryOp},
};

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
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum RValInstruct {
    // Use a value
    Use(Value, Option<Span>),
    /// Binary arithmetic operation
    Binary {
        op: BinaryOp,
        lhs: Box<RValInstruct>,
        rhs: Box<RValInstruct>,
        span: Span,
    },
    /// Unary operation
    Unary {
        op: UnaryOp,
        rhs: Box<RValInstruct>,
        span: Span,
    },
    // Logical operation
    Logical {
        op: LogicalOp,
        lhs: Box<RValInstruct>,
        rhs: Box<RValInstruct>,
        span: Span,
    },
}

impl Display for BinaryOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            BinaryOp::Arithmetic(arith) => match arith {
                ArithOp::Add => write!(f, "+"),
                ArithOp::Sub => write!(f, "-"),
                ArithOp::Mul => write!(f, "*"),
                ArithOp::Div => write!(f, "/"),
                ArithOp::Mod => write!(f, "%"),
            },
            BinaryOp::Compare(cmp) => match cmp {
                CmpOp::Eq => write!(f, "=="),
                CmpOp::Ne => write!(f, "!="),
                CmpOp::Lt => write!(f, "<"),
                CmpOp::Le => write!(f, "<="),
                CmpOp::Gt => write!(f, ">"),
                CmpOp::Ge => write!(f, ">="),
            },
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
            LogicalOp::And => write!(f, "and"),
            LogicalOp::Or => write!(f, "or"),
        }
    }
}

impl Display for RValInstruct {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Use(v, _) => write!(f, "{}", v),
            Self::Binary { op, lhs, rhs, .. } => write!(f, "{} {} {}", lhs, op, rhs),
            Self::Unary { op, rhs, .. } => write!(f, "{}{}", op, rhs),
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
        }
    }
}
