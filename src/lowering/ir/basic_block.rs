use std::fmt::{self, Display, Formatter};

use super::instruction::LValInstruct;
use super::value::Value;
use crate::lowering::ir::instruction::RValInstruct;
use crate::parser::ast::Span;

pub type BlockId = usize;

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
        args: Vec<Value>,
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

pub fn write_block_ir<W: std::fmt::Write>(
    block: &BasicBlock,
    f: &mut W,
    indent: usize,
) -> std::fmt::Result {
    let inner_indent = " ".repeat(indent + 4);

    writeln!(f, "{}block{}: {{", " ".repeat(indent), block.id)?;
    for inst in &block.instructions {
        writeln!(f, "{}{}", inner_indent, inst)?;
    }
    writeln!(f, "{}{}", inner_indent, block.terminator)?;
    writeln!(f, "{}}}", " ".repeat(indent))
}

impl Display for Terminator {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Return(Some(v), _) => write!(f, "return {}", v),
            Self::Return(None, _) => write!(f, "return"),
            Self::Branch { target, args, .. } => {
                let args_str = args
                    .iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(f, "br block{} [{}]", target, args_str)
            }
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
                write!(f, "{} = call {}({})", return_dest, callee, args_str)
            }
        }
    }
}

impl Display for BasicBlock {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write_block_ir(self, f, 0)
    }
}
