//! Variable-based Intermediate Representation (IR) for Mathic
//!
//! Simplified IR that's lower than AST but still high-level:
//! - Uses named variables instead of SSA registers
//! - Mutable variables (Assign instruction)
//! - Basic blocks with explicit control flow
//! - Easier to lower to MLIR than SSA form

use std::{collections::HashMap, fmt};

use function::{Function, write_function_ir};

use crate::parser::ast::declaration::FuncDecl;

pub mod basic_block;
pub mod function;
pub mod instruction;
pub mod types;
pub mod value;

#[derive(Debug, Clone, Default)]
#[allow(dead_code)]
pub struct DeclTable {
    pub functions: HashMap<String, FuncDecl>,
}

#[derive(Debug, Default)]
pub struct Ir {
    pub functions: Vec<Function>,
}

#[derive(Debug, Default)]
pub struct IrBuilder {
    decl_table: DeclTable,
    functions: Vec<Function>,
}

impl IrBuilder {
    pub fn new() -> Self {
        Self {
            decl_table: DeclTable::default(),
            functions: Vec::new(),
        }
    }

    pub fn add_func_decl(&mut self, func: FuncDecl) {
        self.decl_table.functions.insert(func.name.clone(), func);
    }

    pub fn add_function(&mut self, func: Function) {
        self.functions.push(func);
    }

    pub fn build(self) -> Ir {
        Ir {
            functions: self.functions,
        }
    }
}

impl fmt::Display for Ir {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for func in &self.functions {
            write_function_ir(func, f, 0)?;
        }

        Ok(())
    }
}
