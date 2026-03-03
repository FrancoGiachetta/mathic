pub mod ast_lowering;
pub mod ir;

use crate::{
    diagnostics::LoweringError,
    lowering::ast_lowering::statement,
    parser::ast::{Program, declaration::FuncDecl},
};
use ir::{Ir, function::Function};

pub fn lower_program(program: &Program) -> Result<Ir, LoweringError> {
    let mut ir = Ir::new();

    for func in program.funcs.iter() {
        lower_entry_point(func, &mut ir)?;
    }

    Ok(ir)
}

fn lower_entry_point(func: &FuncDecl, ir: &mut Ir) -> Result<(), LoweringError> {
    let FuncDecl {
        name,
        params,
        body,
        span,
        return_ty,
    } = func;

    let mut ir_func = Function::new(name.clone(), params, *return_ty, span.clone());

    for stmt in body {
        statement::lower_stmt(stmt, &mut ir_func)?;
    }

    ir.add_function(ir_func);

    Ok(())
}
