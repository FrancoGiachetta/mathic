//! Functions in the IR

use std::collections::HashSet;

use super::basic_block::{BasicBlock, BlockId};
use crate::{
    lowering::ir::basic_block::Terminator,
    parser::ast::{Span, declaration::Param as AstParam},
};

/// A function in the IR
#[derive(Debug)]
#[allow(dead_code)]
pub struct Function {
    pub name: String,
    pub params: Vec<Param>,
    /// Local variables declared in this function (from Let instructions)
    pub locals: HashSet<String>,
    pub entry_block: BasicBlock,
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

impl Function {
    /// Create a new function
    pub fn new(name: String, span: Span) -> Self {
        Self {
            name,
            params: Vec::new(),
            locals: HashSet::new(),
            basic_blocks: Vec::new(),
            entry_block: BasicBlock::new(0, Terminator::Return(None, None)),
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

    /// Add a local variable
    pub fn add_local(&mut self, name: String) {
        self.locals.insert(name);
    }

    /// Add a basic block
    pub fn add_block(&mut self, block: BasicBlock) -> BlockId {
        let id = block.id;
        self.basic_blocks.push(block);
        id
    }
}
