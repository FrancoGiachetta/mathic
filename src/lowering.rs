pub mod ast_lowering;
pub mod ir;

use crate::{
    diagnostics::LoweringError,
    lowering::{
        ast_lowering::statement,
        ir::{IrBuilder, function::FunctionBuilder},
    },
    parser::ast::{
        Program,
        declaration::{DeclStmt, FuncDecl},
        statement::StmtKind,
    },
};
use ir::Ir;

pub fn lower_program(program: &Program) -> Result<Ir, LoweringError> {
    let mut ir_builder = IrBuilder::new();

    // Save function's declaration. This for on-demand lowering, allowing
    // to reference function no yet declared. For example, a function call
    // of a not yet declared function.
    for f in program.funcs.iter() {
        ir_builder.add_func_decl(f.clone());

        // FUTURE: do the same for structs, enums, etc
    }

    for func in program.funcs.iter() {
        lower_entry_point(func, &mut ir_builder)?;
    }

    Ok(ir_builder.build())
}

fn lower_entry_point(
    func_decl: &FuncDecl,
    ir_builder: &mut IrBuilder,
) -> Result<(), LoweringError> {
    let FuncDecl {
        name,
        params,
        body,
        span,
        return_ty,
    } = func_decl;

    let mut func_builder =
        FunctionBuilder::new(name.clone(), params, *return_ty, ir_builder, *span);

    // Save function's declaration. This for on-demand lowering, allowing
    // to reference function no yet declared. For example, a function call
    // of a not yet declared function.
    for stmt in body.iter() {
        if let StmtKind::Decl(DeclStmt::Func(f)) = &stmt.kind {
            func_builder.add_func_decl(f.clone());
        }

        // FUTURE: do the same for structs, enums, etc
    }

    for stmt in body {
        statement::lower_stmt(&mut func_builder, stmt)?;
    }

    let func = func_builder.build();

    ir_builder.add_function(func);

    Ok(())
}
