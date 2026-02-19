//! Variable-based Intermediate Representation (IR) for Mathic
//!
//! Simplified IR that's lower than AST but still high-level:
//! - Uses named variables instead of SSA registers
//! - Mutable variables (Assign instruction)
//! - Basic blocks with explicit control flow
//! - Easier to lower to MLIR than SSA form

use function::Function;

pub mod basic_block;
pub mod function;
pub mod instruction;
pub mod value;

#[derive(Debug)]
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
