//! Basic blocks and terminators

use super::instruction::Instruction;
use super::value::Value;
use crate::parser::ast::Span;

/// Block identifier
pub type BlockId = usize;

/// A basic block in the control flow graph
#[derive(Debug)]
#[allow(dead_code)]
pub struct BasicBlock {
    pub id: BlockId,
    pub instructions: Vec<Instruction>,
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
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Terminator {
    /// Return from function (optional value)
    Return(Option<Value>, Option<Span>),
    /// Unconditional branch
    Branch(BlockId, Option<Span>),
    /// Conditional branch
    CondBranch {
        condition: Value,
        then_block: BlockId,
        else_block: BlockId,
        span: Option<Span>,
    },
    /// Unreachable code
    Unreachable(Option<Span>),
}
