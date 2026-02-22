use std::collections::HashMap;

use super::basic_block::{BasicBlock, BlockId, write_block_ir};
use crate::{
    lowering::{
        error::LoweringError,
        ir::{basic_block::Terminator, instruction::LValInstruct},
    },
    parser::ast::Span,
};

/// A function in the IR
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Function {
    pub name: String,
    pub sym_table: SymbolTable,
    pub basic_blocks: Vec<BasicBlock>,
    pub span: Span,
}

#[derive(Debug, Clone, Copy)]
pub enum LocalKind {
    Param,
    Temp,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Local {
    pub local_idx: usize,
    pub kind: LocalKind,
    pub debug_name: Option<String>,
}

#[derive(Debug, Clone, Default)]
#[allow(dead_code)]
pub struct SymbolTable {
    pub locals: Vec<Local>,
    pub functions: Vec<Function>,
    pub local_indexes: HashMap<String, usize>,
    pub function_indexes: HashMap<String, usize>,
}

impl Function {
    /// Create a new function
    pub fn new(name: String, span: Span) -> Self {
        Self {
            name,
            sym_table: Default::default(),
            basic_blocks: vec![BasicBlock::new(0, Terminator::Return(None, None), None)],
            span,
        }
    }

    /// Adds a user-defined local.
    pub fn add_local(
        &mut self,
        debug_name: Option<String>,
        span: Option<Span>,
        kind: LocalKind,
    ) -> Result<usize, LoweringError> {
        if let Some(name) = &debug_name
            && self.sym_table.local_indexes.contains_key(name)
        {
            return Err(LoweringError::DuplicateDeclaration {
                name: name.clone(),
                span: span.unwrap(),
            });
        }

        let idx = self.sym_table.locals.len();

        self.sym_table.locals.push(Local {
            local_idx: idx,
            kind,
            debug_name: debug_name.clone(),
        });

        if let Some(name) = debug_name {
            self.sym_table.local_indexes.insert(name, idx);
        }

        Ok(idx)
    }

    pub fn add_function(&mut self, func: Function) -> usize {
        let idx = self.sym_table.functions.len();

        self.sym_table.functions.push(func);

        idx
    }

    pub fn get_local_idx_from_name(&self, name: &str) -> Option<usize> {
        self.sym_table.local_indexes.get(name).copied()
    }

    /// Add a basic block
    pub fn add_block(&mut self, terminator: Terminator, span: Option<Span>) -> BlockId {
        let id = self.basic_blocks.len();

        self.basic_blocks.push(BasicBlock {
            id,
            instructions: Vec::new(),
            terminator,
            span,
        });

        id
    }

    pub fn push_instruction(&mut self, inst: LValInstruct) {
        let last_index = self.basic_blocks.len() - 1;
        self.basic_blocks[last_index].instructions.push(inst);
    }

    pub fn get_basic_block_mut(&mut self, idx: usize) -> &mut BasicBlock {
        &mut self.basic_blocks[idx]
    }

    pub fn last_block_idx(&self) -> BlockId {
        self.basic_blocks.len() - 1
    }
}

pub fn write_function_ir<W: std::fmt::Write>(
    func: &Function,
    f: &mut W,
    indent: usize,
) -> std::fmt::Result {
    let indent_str = " ".repeat(indent);

    let params = func
        .sym_table
        .locals
        .iter()
        .filter(|local| matches!(local.kind, LocalKind::Param))
        .map(|p| format!("%{}", p.local_idx))
        .collect::<Vec<_>>()
        .join(", ");

    writeln!(f, "{}df {}({}) -> i64 {{", indent_str, func.name, params)?;
    for nested_func in func.sym_table.functions.iter() {
        write_function_ir(nested_func, f, indent + 4)?;
    }
    for block in func.basic_blocks.iter() {
        write_block_ir(block, f, indent + 4)?;
    }
    writeln!(f, "{}}}\n", indent_str)
}
