use super::types::MathicType;
use super::value::Value;
use crate::{
    lowering::ir::{symbols::TypeIndex, value::ValueModifier},
    parser::{
        Span,
        ast::expression::{BinaryOp, LogicalOp, UnaryOp},
    },
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
        modifier: Vec<ValueModifier>,
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
    Init {
        init_inst: InitInstruct,
        span: Span,
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

#[derive(Debug, Clone)]
pub enum InitInstruct {
    StructInit { fields: Vec<RValInstruct> },
}

/// MATHIR's representation of RValue instruction.
///
/// An RValue instruction represents the evaluation of an expression used as
/// the value of an LValue instruction.
#[derive(Debug, Clone)]
pub struct RValInstruct {
    pub kind: RValueKind,
    pub ty: TypeIndex,
}
