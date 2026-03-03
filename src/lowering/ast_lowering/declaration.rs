use crate::{
    diagnostics::LoweringError,
    lowering::{
        ast_lowering::{expression, statement},
        ir::{
            function::{Function, LocalKind},
            instruction::LValInstruct,
        },
    },
    parser::{
        Span,
        ast::declaration::{FuncDecl, VarDecl},
    },
};

pub fn lower_var_declaration(
    func: &mut Function,
    stmt: &VarDecl,
    span: Span,
) -> Result<(), LoweringError> {
    let VarDecl {
        name,
        expr,
        ty: var_ty,
    } = stmt;

    let (init, expr_ty) = expression::lower_expr(func, expr, Some(*var_ty))?;

    if expr_ty != *var_ty {
        return Err(LoweringError::MismatchedType {
            expected: *var_ty,
            found: expr_ty,
            span,
        });
    }

    let local_idx = func.add_local(Some(name.clone()), *var_ty, Some(span), LocalKind::Temp)?;

    func.push_instruction(LValInstruct::Let {
        local_idx,
        init,
        span: Some(span),
    });

    Ok(())
}

pub fn lower_inner_function(
    func: &mut Function,
    stmt: &FuncDecl,
    span: Span,
) -> Result<(), LoweringError> {
    let FuncDecl {
        name,
        params,
        body,
        return_ty,
        ..
    } = stmt;

    let mut inner_func = Function::new(name.clone(), params, *return_ty, span);

    for stmt in body.iter() {
        statement::lower_stmt(stmt, &mut inner_func)?;
    }

    func.add_function(inner_func);

    Ok(())
}
