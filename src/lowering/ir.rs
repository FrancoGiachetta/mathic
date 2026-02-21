//! Variable-based Intermediate Representation (IR) for Mathic
//!
//! Simplified IR that's lower than AST but still high-level:
//! - Uses named variables instead of SSA registers
//! - Mutable variables (Assign instruction)
//! - Basic blocks with explicit control flow
//! - Easier to lower to MLIR than SSA form

use std::fmt;

use function::{Function, write_function_ir};

pub mod basic_block;
pub mod function;
pub mod instruction;
pub mod value;

#[derive(Debug, Default)]
#[allow(dead_code)]
pub struct Ir {
    pub functions: Vec<Function>,
}

impl Ir {
    /// Create a new empty IR
    pub fn new() -> Self {
        Self {
            functions: Vec::new(),
        }
    }

    /// Add a function to the IR
    pub fn add_function(&mut self, func: Function) {
        self.functions.push(func);
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
