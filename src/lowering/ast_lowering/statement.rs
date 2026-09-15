use crate::{
    diagnostics::LoweringError,
    lowering::{
        ast_lowering::{declaration::lower_sym_decl, lower_ast_type},
        ir::{
            Builder,
            adts::{Adt, StructAdt, StructField},
            basic_block::{BlockId, Terminator},
            function::FunctionBuilder,
        },
    },
    parser::{
        Span,
        ast::{
            declaration::{DeclStmt, StructDecl},
            statement::{BlockStmt, Stmt, StmtKind},
        },
    },
};

use super::control_flow::{lower_for, lower_if, lower_while};
use super::declaration::{lower_function, lower_var_declaration};
use super::expression::lower_expr;

pub fn lower_stmt(func: &mut FunctionBuilder, stmt: &Stmt) -> Result<(), LoweringError> {
    match &stmt.kind {
        StmtKind::Decl(decl) => lower_declaration(func, decl, &stmt.span)?,
        StmtKind::Return(expr) => {
            let (value, value_ty_idx) = lower_expr(func, expr, Some(func.return_ty))?;

            if value_ty_idx != func.return_ty {
                return Err(LoweringError::MismatchedReturnType {
                    expected: func.get_type(func.return_ty, stmt.span)?,
                    found: func.get_type(value_ty_idx, stmt.span)?,
                    span: stmt.span,
                });
            }

            func.get_basic_block_mut(func.last_block_idx()).terminator =
                Terminator::Return(Some(value), Some(stmt.span));
        }
        StmtKind::Block(block_stmt) => {
            let curr_block_idx = func.last_block_idx();

            func.get_basic_block_mut(curr_block_idx).terminator = Terminator::Branch {
                target: curr_block_idx + 1,
                block_args: Vec::new(),
                span: None,
            };

            let _ = lower_block(
                func,
                block_stmt,
                Terminator::Branch {
                    target: curr_block_idx + 2,
                    block_args: Vec::new(),
                    span: None,
                },
            )?;
        }
        StmtKind::Expr(expr) => {
            let _ = lower_expr(func, expr, None)?;
        }
        StmtKind::If(if_stmt) => lower_if(func, if_stmt)?,
        StmtKind::While(while_stmt) => lower_while(func, while_stmt, stmt.span)?,
        StmtKind::For(for_stmt) => lower_for(func, for_stmt, stmt.span)?,
    }

    Ok(())
}

fn lower_declaration(
    func: &mut FunctionBuilder,
    stmt: &DeclStmt,
    span: &Span,
) -> Result<(), LoweringError> {
    match stmt {
        DeclStmt::Var(var_decl) => {
            lower_var_declaration(func, var_decl, *span)?;
        }
        DeclStmt::Struct(struct_decl) => {
            let _ = lower_struct(func, struct_decl)?;
        }
        DeclStmt::ExpandDecl(expand_block_) => unimplemented!(),
        DeclStmt::Sym(sym_decl) => lower_sym_decl(func, sym_decl, *span)?,
        DeclStmt::Func(func_decl) => lower_function(func, func_decl)?,
    }

    Ok(())
}

pub fn lower_struct(
    builder: &mut dyn Builder,
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
            ty: lower_ast_type(builder, &field.ty, field.span)?,
            _is_pub: field.is_pub,
        });
    }

    let idx = builder.add_adt(adt.name.clone(), Adt::Struct(adt));

    Ok(idx)
}

pub fn lower_block(
    func: &mut FunctionBuilder,
    block: &BlockStmt,
    terminator: Terminator,
) -> Result<BlockId, LoweringError> {
    let old_sym_table = func.sym_table.clone();

    let block_id = func.add_block(terminator, Some(block.span));

    for s in block.stmts.iter() {
        lower_stmt(func, s)?;
    }

    func.sym_table = old_sym_table;

    Ok(block_id)
}
