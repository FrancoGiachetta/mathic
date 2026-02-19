//! Lowering module: AST to IR to MLIR
//!
//! This module handles the transformation pipeline:
//! AST → IR → MLIR → LLVM

pub mod control_flow;
pub mod expression;
pub mod ir;
pub mod statement;

use crate::parser::ast::{Program, declaration::FuncDecl};
use ir::{Ir, function::Function};

pub struct Lowerer;

impl Lowerer {
    pub fn new() -> Self {
        Self
    }

    pub fn lower_program(&mut self, program: Program) -> Ir {
        let mut ir = Ir::new();

        for func in program.funcs {
            self.lower_function(func, &mut ir);
        }

        ir
    }

    fn lower_function(&self, func: FuncDecl, ir: &mut Ir) {
        let mut ir_func = Function::new(func.name, func.span);

        for param in func.params {
            ir_func.add_param(param);
        }

        for stmt in &func.body {
            self.lower_stmt(stmt, &mut ir_func);
        }

        ir.add_function(ir_func);
    }
}
