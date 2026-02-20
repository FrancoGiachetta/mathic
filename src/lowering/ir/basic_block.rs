//! Basic blocks and terminators

use super::instruction::LValInstruct;
use super::value::Value;
use crate::lowering::ir::instruction::RValInstruct;
use crate::parser::ast::Span;

/// Block identifier
pub type BlockId = usize;

/// A basic block in the control flow graph
#[derive(Debug)]
#[allow(dead_code)]
pub struct BasicBlock {
    pub id: BlockId,
    pub instructions: Vec<LValInstruct>,
    pub terminator: Terminator,
}

impl BasicBlock {
    pub fn new(id: BlockId, terminator: Terminator) -> Self {
        Self {
            id,
            instructions: Vec::new(),
            terminator,
        }
    }
}

/// Terminator instructions that end a basic block
#[derive(Debug)]
#[allow(dead_code)]
pub enum Terminator {
    /// Return from function (optional value)
    Return(Option<RValInstruct>, Option<Span>),
    /// Unconditional branch
    Branch(BlockId, Option<Span>),
    /// Conditional branch
    CondBranch {
        condition: RValInstruct,
        then_block: BlockId,
        else_block: BlockId,
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
        dest_block: usize,
    },
}
