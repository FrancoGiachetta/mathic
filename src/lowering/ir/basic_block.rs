use super::instruction::LValInstruct;
use super::value::Value;
use crate::lowering::ir::instruction::RValInstruct;
use crate::lowering::ir::types::MathicType;
use crate::parser::Span;

pub type BlockId = usize;

/// MATHIR's representation of a basic block.
///
/// A basic block is a list of lvalue instructions which **always** ends with a
/// terminator instruction.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct BasicBlock {
    pub id: BlockId,
    pub instructions: Vec<LValInstruct>,
    pub terminator: Terminator,
    pub span: Option<Span>,
}

impl BasicBlock {
    pub fn new(id: BlockId, terminator: Terminator, span: Option<Span>) -> Self {
        Self {
            id,
            instructions: Vec::new(),
            terminator,
            span,
        }
    }
}

/// MATHIR's representation of a terminator.
///
/// A terminator is a type of instruction that represents the ending of a block.
/// A terminator me represent the return of a function or an uncoditional jump
/// for example.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Terminator {
    /// Return from function
    Return(Option<RValInstruct>, Option<Span>),
    /// Unconditional branch
    Branch { target: BlockId, span: Option<Span> },
    /// Conditional branch
    CondBranch {
        condition: RValInstruct,
        true_block: BlockId,
        false_block: BlockId,
        span: Option<Span>,
    },
    /// Unreachable code
    Unreachable(Option<Span>),
    /// Function call
    Call {
        callee: String,
        args: Vec<RValInstruct>,
        span: Option<Span>,
        return_dest: Value,
        return_ty: MathicType,
        dest_block: usize,
    },
}
