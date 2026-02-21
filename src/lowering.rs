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

        for func in program.funcs.iter() {
            self.lower_entry_point(func, &mut ir);
        }

        ir
    }

    fn lower_entry_point(&self, func: &FuncDecl, ir: &mut Ir) {
        let mut ir_func = Function::new(func.name.clone(), func.span.clone());

        for param in func.params.iter() {
            ir_func.add_local(Some(param.name.clone()), LocalKind::Param);
        }

        for stmt in &func.body {
            self.lower_stmt(stmt, &mut ir_func);
        }

        ir.add_function(ir_func);
    }
}
