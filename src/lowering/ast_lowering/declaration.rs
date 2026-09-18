use std::collections::HashSet;

use crate::{
    diagnostics::LoweringError,
    lowering::{
        ast_lowering::{expression, lower_ast_type, statement},
        ir::{
            Builder,
            function::{FunctionBuilder, LocalKind},
            instruction::{LValInstruct, RValueKind},
            symbols::TypeIndex,
            types::MathicType,
            value::Value,
        },
    },
    parser::{
        Span,
        ast::{
            declaration::{DeclStmt, ExpandDecl, FuncDecl, SymDecl, VarDecl},
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
    let var_ty_idx = lower_ast_type(func, var_ty, span)?;
    let (init, expr_ty_idx) = expression::lower_expr(func, expr, Some(var_ty_idx))?;

    let var_ty = func.get_type(var_ty_idx, span)?;
    let expr_ty = func.get_type(expr_ty_idx, span)?;
    println!(
        "lower_var_declaration: var_ty = {:?}, expr_ty = {:?}",
        var_ty, expr_ty
    );
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

    // We need to track the symbols used in the symbolic expression.
    if func.get_type(var_ty_idx, span)?.is_symbolic() {
        let symbols = match &init.kind {
            RValueKind::SymbolicBinary { symbols, .. } => symbols.clone(),
            RValueKind::Use {
                value: Value::Symbol { local_idx },
                ..
            } => func.sym_table.locals[*local_idx].symbols.clone(),
            _ => HashSet::with_capacity(0),
        };

        func.sym_table.locals[local_idx].symbols = symbols;
    }

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

    let sym_ty_idx = lower_ast_type(func, ty, span)?;
    let local_idx =
        func.sym_table
            .add_local(Some(name.clone()), sym_ty_idx, Some(span), LocalKind::Sym)?;

    func.sym_table.locals[local_idx].symbols = HashSet::from([local_idx]);

    func.push_instruction(LValInstruct::Sym {
        local_idx,
        sym_name: name.clone(),
        ty: sym_ty_idx,
        span: Some(span),
    });

    Ok(())
}

pub fn lower_expand_block(
    builder: &mut impl Builder,
    expand_block: &ExpandDecl,
) -> Result<(), LoweringError> {
    let ExpandDecl {
        adt_name, methods, ..
    } = expand_block;

    let assoc_ty =
        builder
            .get_user_def_type(&adt_name.join("::"))
            .ok_or(LoweringError::UndeclaredType {
                span: adt_name.span,
            })?;

    let old_self_ty = builder.get_self_ty_idx();

    builder.set_self_ty_idx(Some(assoc_ty));
    for m in methods {
        lower_function(builder, m, Some(assoc_ty))?;
    }
    builder.set_self_ty_idx(old_self_ty);

    Ok(())
}

pub fn lower_function(
    builder: &mut impl Builder,
    stmt: &FuncDecl,
    method_of: Option<TypeIndex>,
) -> Result<(), LoweringError> {
    let FuncDecl {
        name,
        params,
        body,
        return_ty,
        ..
    } = stmt;

    let func_name = builder.get_ir_builder().module_name.clone();
    let mangled_function_name = builder.get_mangled_name(&func_name, name);

    let mut inner_func = FunctionBuilder::new(
        mangled_function_name,
        params,
        match return_ty {
            Some(ty) => lower_ast_type(builder, ty, stmt.span)?,
            None => builder.get_or_insert_type_idx(MathicType::Void),
        },
        builder.get_ir_builder(),
        stmt.span,
        false,
    )?;

    // Save function's declaration. This for on-demand lowering, allowing
    // to reference function no yet declared. For example, a function call
    // of a not yet declared function.
    for stmt in body.iter() {
        if let StmtKind::Decl(decl_stmt) = &stmt.kind {
            match decl_stmt {
                DeclStmt::Func(f) => inner_func.add_function_decl(f.clone(), None, None)?,
                DeclStmt::Struct(s) => inner_func.add_struct_decl(s.clone(), None)?,
                DeclStmt::ExpandBlock(expand_decl) => {
                    let ExpandDecl {
                        adt_name,
                        methods,
                        span,
                    } = expand_decl;

                    // The ADT must have been defined before the `expand` block.
                    let adt_ty = inner_func
                        .get_user_def_type(&adt_name.join("::"))
                        .ok_or(LoweringError::UndeclaredType { span: *span })?;

                    for m in methods {
                        inner_func.add_function_decl(m.clone(), Some(adt_ty), None)?
                    }
                }
                _ => {}
            }
        }
    }

    for stmt in body.iter() {
        statement::lower_stmt(&mut inner_func, stmt)?;
    }

    let inner_func = inner_func.build();

    builder.add_function(inner_func, method_of);

    Ok(())
}
