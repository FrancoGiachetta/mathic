//! Lowering module: AST to IR to MLIR
//!
//! This module handles the transformation pipeline:
//! AST → IR → MLIR → LLVM

pub mod ir;

use crate::parser::ast::Program;
use ir::Ir;

/// The Lowerer is responsible for converting AST into IR
#[derive(Debug)]
#[allow(dead_code)]
pub struct Lowerer {
    /// The IR being constructed
    ir: Ir,
}

impl Lowerer {
    /// Create a new Lowerer
    pub fn new() -> Self {
        Self { ir: Ir::new() }
    }

    /// Lower an AST Program into IR
    pub fn lower_program(&mut self, program: Program) -> Ir {
        // TODO: Implement lowering logic
        // For each function in program.funcs:
        //   - Create Function struct
        //   - Lower function body into basic blocks
        //   - Add function to self.ir

        // Return the completed IR

        Ir::new()
    }

    /// Get a reference to the IR being built
    pub fn ir(&self) -> &Ir {
        &self.ir
    }
}
