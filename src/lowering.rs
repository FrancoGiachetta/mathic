pub mod ast_lowering;
pub mod ir;

use crate::{
    lowering::ir::function::LocalKind,
    parser::ast::{Program, declaration::FuncDecl},
};
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
            ir_func.add_local(param.name, LocalKind::Param, param.span);
        }

        for stmt in &func.body {
            self.lower_stmt(stmt, &mut ir_func);
        }

        ir.add_function(ir_func);
    }
}
