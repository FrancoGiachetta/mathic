use crate::{
    diagnostics::LoweringError,
    lowering::{
        ast_lowering::{expression, statement},
        ir::{
            adts::{Adt, StructAdt, StructField},
            function::{FunctionBuilder, LocalKind},
            instruction::LValInstruct,
            types::{MathicType, lower_inner_ast_type},
        },
    },
    parser::{
        Span,
        ast::{
            declaration::{DeclStmt, FuncDecl, StructDecl, SymDecl, VarDecl},
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
    let var_ty_idx = lower_inner_ast_type(func, var_ty, span)?;
    let (init, expr_ty_idx) = expression::lower_expr(func, expr, Some(var_ty_idx))?;

    let var_ty = func.get_type(var_ty_idx, span)?;
    let expr_ty = func.get_type(expr_ty_idx, span)?;

    if expr_ty_idx != var_ty_idx {
        return Err(LoweringError::MismatchedType {
            expected: var_ty,
            found: expr_ty,
            span,
        });
    }

    let local_idx =
        func.sym_table
            .add_local(Some(name.clone()), var_ty_idx, Some(span), LocalKind::Temp)?;

    func.push_instruction(LValInstruct::Let {
        local_idx,
        init,
        span: Some(span),
    });

    Ok(())
}

pub fn lower_sym_decl(
    func: &mut FunctionBuilder,
    sym_decl: &SymDecl,
    span: Span,
) -> Result<(), LoweringError> {
    let SymDecl { name, ty } = sym_decl;

    let sym_ty_idx = lower_inner_ast_type(func, ty, span)?;
    let local_idx =
        func.sym_table
            .add_local(Some(name.clone()), sym_ty_idx, Some(span), LocalKind::Sym)?;

    func.push_instruction(LValInstruct::Sym {
        local_idx,
        sym_name: name.clone(),
        ty: sym_ty_idx,
        span: Some(span),
    });

    Ok(())
}

pub fn lower_inner_struct(
    func: &mut FunctionBuilder,
    struct_decl: &StructDecl,
) -> Result<usize, LoweringError> {
    let StructDecl { name, fields, span } = struct_decl;

    let mut adt = StructAdt {
        name: name.clone(),
        fields: Vec::new(),
        _span: *span,
    };

    for field in fields {
        adt.fields.push(StructField {
            name: field.name.clone(),
            ty: lower_inner_ast_type(func, &field.ty, field.span)?,
            _is_pub: field.is_pub,
        });
    }

    let idx = func.add_adt(adt.name.clone(), Adt::Struct(adt));

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
            Some(ty) => lower_inner_ast_type(func, ty, span)?,
            None => func.get_or_insert_global_type_idx(MathicType::Void),
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
