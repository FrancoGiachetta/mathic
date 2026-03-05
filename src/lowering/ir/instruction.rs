use std::fmt::{self, Display, Formatter};

use super::types::MathicType;
use super::value::Value;
use crate::parser::{
    Span,
    ast::expression::{ArithOp, BinaryOp, CmpOp, LogicalOp, UnaryOp},
};

/// MATHIR's representation of LValue instruction.
///
/// An LValue instruction represents either the declaration of a variable or an
/// assigment to it.
#[derive(Debug, Clone)]
pub enum LValInstruct {
    Let {
        local_idx: usize,
        init: RValInstruct,
        span: Option<Span>,
    },
    Assign {
        local_idx: usize,
        value: RValInstruct,
        span: Option<Span>,
    },
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum RValueKind {
    Use {
        value: Value,
        span: Option<Span>,
    },
    Binary {
        op: BinaryOp,
        lhs: Box<RValInstruct>,
        rhs: Box<RValInstruct>,
        span: Span,
    },
    Unary {
        op: UnaryOp,
        rhs: Box<RValInstruct>,
        span: Span,
    },
    Logical {
        op: LogicalOp,
        lhs: Box<RValInstruct>,
        rhs: Box<RValInstruct>,
        span: Span,
    },
}

/// MATHIR's representation of RValue instruction.
///
/// An RValue instruction represents the evaluation of an expression used as
/// the value of an LValue instruction.
#[derive(Debug, Clone)]
pub struct RValInstruct {
    pub kind: RValueKind,
    pub ty: MathicType,
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

impl Display for RValueKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Use { value, .. } => write!(f, "{}", value),
            Self::Binary { op, lhs, rhs, .. } => write!(f, "{} {} {}", lhs, op, rhs),
            Self::Unary { op, rhs, .. } => write!(f, "{}{}", op, rhs),
            Self::Logical { op, lhs, rhs, .. } => write!(f, "{} {} {}", lhs, op, rhs),
        }
    }
}

impl Display for RValInstruct {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.kind)
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
