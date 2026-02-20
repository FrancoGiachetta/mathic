use std::{
    collections::HashMap,
    fmt::{self, Display, Formatter},
};

use super::basic_block::{BasicBlock, BlockId};
use crate::{
    lowering::ir::{basic_block::Terminator, instruction::LValInstruct},
    parser::ast::Span,
};

/// A function in the IR
#[derive(Debug)]
#[allow(dead_code)]
pub struct Function {
    pub name: String,
    pub params: Vec<Param>,
    pub locals: Vec<Local>,
    pub local_indexes: HashMap<String, usize>,
    pub basic_blocks: Vec<BasicBlock>,
    pub span: Span,
}

/// Function parameter
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Param {
    pub name: String,
    pub index: usize,
    pub span: Span,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalKind {
    Param,
    Temp,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Local {
    pub local_idx: usize,
    pub kind: LocalKind,
    pub debug_name: Option<String>,
    pub span: Option<Span>,
}

impl Function {
    /// Create a new function
    pub fn new(name: String, span: Span) -> Self {
        Self {
            name,
            params: Vec::new(),
            locals: Vec::new(),
            local_indexes: HashMap::new(),
            basic_blocks: vec![BasicBlock::new(0, Terminator::Return(None, None))],
            span,
        }
    }

    /// Adds a user-defined local.
    pub fn add_local(&mut self, name: String, kind: LocalKind, span: Span) -> usize {
        let idx = self.locals.len();
        self.locals.push(Local {
            local_idx: idx,
            kind,
            debug_name: Some(name.clone()),
            span: Some(span),
        });
        self.local_indexes.insert(name, idx);

        idx
    }

    /// Adds a temp local.
    ///
    /// This is reserved for compiler created locals.
    pub fn add_local_temp(&mut self) -> usize {
        let idx = self.locals.len();
        self.locals.push(Local {
            local_idx: idx,
            kind: LocalKind::Temp,
            debug_name: None,
            span: None,
        });

        idx
    }

    pub fn get_local_idx_from_name(&self, name: &str) -> Option<usize> {
        self.local_indexes.get(name).copied()
    }

    /// Add a basic block
    pub fn add_block(&mut self, block: BasicBlock) -> BlockId {
        let id = block.id;
        self.basic_blocks.push(block);
        id
    }

    pub fn push_instruction(&mut self, inst: LValInstruct) {
        let last_index = self.basic_blocks.len() - 1;
        self.basic_blocks[last_index].instructions.push(inst);
    }

    pub fn last_basic_block(&mut self) -> &mut BasicBlock {
        let last_index = self.basic_blocks.len() - 1;
        &mut self.basic_blocks[last_index]
    }

    pub fn last_block_idx(&self) -> BlockId {
        self.basic_blocks.len() - 1
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let params = self
            .params
            .iter()
            .map(|p| p.name.clone())
            .collect::<Vec<_>>()
            .join(", ");

        writeln!(f, "df {}({}) -> i64 {{", &self.name, params)?;
        for block in self.basic_blocks.iter() {
            writeln!(f, "    {}", block)?;
        }
        write!(f, "}}")
    }
}
