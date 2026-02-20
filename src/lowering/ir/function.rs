//! Functions in the IR

use std::collections::HashMap;

use super::basic_block::{BasicBlock, BlockId};
use crate::{
    lowering::ir::{basic_block::Terminator, instruction::LValInstruct},
    parser::ast::{Span, declaration::Param as AstParam},
};

/// A function in the IR
#[derive(Debug)]
#[allow(dead_code)]
pub struct Function {
    name: String,
    params: Vec<Param>,
    locals: Vec<Local>,
    local_indexes: HashMap<String, usize>,
    basic_blocks: Vec<BasicBlock>,
    span: Span,
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

    /// Add a parameter
    pub fn add_param(&mut self, param: AstParam) -> usize {
        let index = self.params.len();
        self.params.push(Param {
            name: param.name,
            index,
            span: param.span,
        });
        index
    }

    /// Adds a user-defined local.
    pub fn add_local(&mut self, name: String, kind: LocalKind) -> usize {
        let idx = self.locals.len();
        self.locals.push(Local {
            local_idx: idx,
            kind,
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
