//! Functions in the IR

use std::collections::HashSet;

use super::basic_block::{BasicBlock, BlockId};
use crate::parser::ast::Span;

/// A function in the IR
#[derive(Debug)]
#[allow(dead_code)]
pub struct Function {
    pub name: String,
    pub params: Vec<Param>,
    /// Local variables declared in this function (from Let instructions)
    pub locals: HashSet<String>,
    pub basic_blocks: Vec<BasicBlock>,
    pub entry_block: BlockId,
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

impl Function {
    /// Create a new function
    pub fn new(name: String, span: Span) -> Self {
        Self {
            name,
            params: Vec::new(),
            locals: HashSet::new(),
            basic_blocks: Vec::new(),
            entry_block: 0,
            span,
        }
    }

    /// Add a parameter
    pub fn add_param(&mut self, name: String, span: Span) -> usize {
        let index = self.params.len();
        self.params.push(Param { name, index, span });
        index
    }

    /// Add a local variable
    pub fn add_local(&mut self, name: String) {
        self.locals.insert(name);
    }

    /// Check if a name is a local variable
    pub fn is_local(&self, name: &str) -> bool {
        self.locals.contains(name)
    }

    /// Check if a name is a parameter
    pub fn is_param(&self, name: &str) -> bool {
        self.params.iter().any(|p| p.name == name)
    }

    /// Add a basic block
    pub fn add_block(&mut self, block: BasicBlock) -> BlockId {
        let id = block.id;
        self.basic_blocks.push(block);
        id
    }

    /// Get a mutable reference to a block
    pub fn block_mut(&mut self, id: BlockId) -> Option<&mut BasicBlock> {
        self.basic_blocks.iter_mut().find(|b| b.id == id)
    }
}
