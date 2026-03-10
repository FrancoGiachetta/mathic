use crate::{
    diagnostics::LoweringError,
    lowering::{
        ast_lowering::{expression, statement},
        ir::{
            adts::{Adt, StructAdt, StructField},
            function::{FunctionBuilder, LocalKind},
            instruction::LValInstruct,
            types::{MathicType, lower_ast_type},
        },
    },
    parser::{
        Span,
        ast::{
            declaration::{DeclStmt, FuncDecl, StructDecl, VarDecl},
            statement::StmtKind,
        },
    },
};

pub fn lower_var_declaration(
    func: &mut FunctionBuilder,
    stmt: &VarDecl,
    span: Span,
) -> Result<(), LoweringError> {
    let VarDecl {
        name,
        expr,
        ty: var_ty,
    } = stmt;
    let var_ty = lower_ast_type(func, var_ty)?;
    let (init, expr_ty) = expression::lower_expr(func, expr, Some(var_ty))?;

    if expr_ty != var_ty {
        return Err(LoweringError::MismatchedType {
            expected: var_ty,
            found: expr_ty,
            span,
        });
    }

    let local_idx =
        func.sym_table
            .add_local(Some(name.clone()), var_ty, Some(span), LocalKind::Temp)?;

    func.push_instruction(LValInstruct::Let {
        local_idx,
        init,
        span: Some(span),
    });

    Ok(())
}

pub fn lower_inner_struct(
    func_builder: &mut FunctionBuilder,
    struct_decl: &StructDecl,
) -> Result<usize, LoweringError> {
    let StructDecl { name, fields, span } = struct_decl;

    let mut adt = StructAdt {
        name: name.clone(),
        fields: Vec::new(),
        span: *span,
    };

    for field in fields {
        adt.fields.push(StructField {
            name: field.name.clone(),
            ty: lower_ast_type(func_builder, &field.ty)?,
            is_pub: field.is_pub,
        });
    }

    let idx = func_builder
        .sym_table
        .add_adt(name.clone(), Adt::Struct(adt));

    Ok(idx)
}

pub fn lower_inner_function(
    func: &mut FunctionBuilder,
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

    let mut inner_func = FunctionBuilder::new(
        name.clone(),
        params,
        match return_ty {
            Some(ty) => lower_ast_type(func, ty)?,
            None => MathicType::Void,
        },
        func.ir_builder,
        span,
    )?;

    // Save function's declaration. This for on-demand lowering, allowing
    // to reference function no yet declared. For example, a function call
    // of a not yet declared function.
    for stmt in body.iter() {
        match &stmt.kind {
            StmtKind::Decl(DeclStmt::Func(f)) => inner_func.decl_table.add_func_decl(f.clone()),
            StmtKind::Decl(DeclStmt::Struct(f)) => inner_func.decl_table.add_struct_decl(f.clone()),
            _ => {}
        }
    }

    for stmt in body.iter() {
        statement::lower_stmt(&mut inner_func, stmt)?;
    }

    let inner_func = inner_func.build();

    func.sym_table.add_function(inner_func);

    Ok(())
}
