use std::fmt::{self, Display, Formatter};

use super::instruction::LValInstruct;
use super::value::Value;
use crate::lowering::ir::function::Local;
use crate::lowering::ir::instruction::RValInstruct;
use crate::parser::ast::Span;

/// Block identifier
pub type BlockId = usize;

/// A basic block in the control flow graph
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Terminator {
    /// Return from function (optional value)
    Return(Option<RValInstruct>, Option<Span>),
    /// Unconditional branch
    Branch {
        target: BlockId,
        params: Vec<Value>,
        span: Option<Span>,
    },
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

impl Display for Terminator {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Return(Some(v), _) => write!(f, "return {}", v),
            Self::Return(None, _) => write!(f, "return"),
            Self::Branch { target, .. } => write!(f, "br block{} []", target),
            Self::CondBranch {
                condition,
                then_block,
                else_block,
                ..
            } => {
                write!(
                    f,
                    "cond_br {} then block{} else block{}",
                    condition, then_block, else_block
                )
            }
            Self::Unreachable(_) => write!(f, "unreachable"),
            Self::Call {
                callee,
                args,
                return_dest,
                ..
            } => {
                let args_str = args
                    .iter()
                    .map(|a| a.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(f, "call {} = {}({})", return_dest, callee, args_str)
            }
        }
    }
}

impl Display for BasicBlock {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "block{}: {{", self.id)?;
        for inst in &self.instructions {
            writeln!(f, "        {}", inst)?;
        }
        writeln!(f, "        {}", self.terminator)?;
        write!(f, "    }}")
    }
}
