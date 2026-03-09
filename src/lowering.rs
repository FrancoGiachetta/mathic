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
use tracing::instrument;

/// Lowering entrypoint.
///
/// Given an AST, this function lowers it and returns a MATHIR. In the process,
/// semantic check are perfomed to verify the correctness of the program.
#[instrument(target = "lowering")]
pub fn lower_program(program: &Program) -> Result<Ir, LoweringError> {
    let start = std::time::Instant::now();
    tracing::info!("Starting lowering phase");
    let mut ir_builder = IrBuilder::new();

    // Save function's declaration. This for on-demand lowering, allowing
    // to reference function no yet declared. For example, a function call
    // of a not yet declared function.
    for f in program.funcs.iter() {
        ir_builder.add_func_decl(f.clone());

        // FUTURE: do the same for structs, enums, etc
    }

    for func in program.funcs.iter() {
        lower_entry_point(&mut ir_builder, func)?;
    }

    tracing::info!("Lowering complete: {:?}", start.elapsed());
    Ok(ir_builder.build())
}

/// Lowers global functions.
#[instrument(target = "lowering", skip(ir_builder))]
fn lower_entry_point(
    ir_builder: &mut IrBuilder,
    func_decl: &FuncDecl,
) -> Result<(), LoweringError> {
    tracing::debug!("Lowering function: {}", func_decl.name);
    let FuncDecl {
        name,
        params,
        body,
        span,
        return_ty,
    } = func_decl;

    let mut func_builder =
        FunctionBuilder::new(name.clone(), params, return_ty.into(), ir_builder, *span);

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
