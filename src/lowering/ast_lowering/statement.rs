use crate::{
    diagnostics::LoweringError,
    lowering::ir::{
        basic_block::{BlockId, Terminator},
        function::Function,
    },
    parser::ast::{
        Span,
        declaration::DeclStmt,
        statement::{BlockStmt, Stmt, StmtKind},
    },
};

use super::control_flow::{lower_for, lower_if, lower_while};
use super::declaration::{lower_inner_function, lower_var_declaration};
use super::expression::lower_expr;

pub fn lower_stmt(stmt: &Stmt, func: &mut Function) -> Result<(), LoweringError> {
    match &stmt.kind {
        StmtKind::Decl(decl) => lower_declaration(func, decl, &stmt.span)?,
        StmtKind::Return(expr) => {
            let value = lower_expr(func, expr)?;
            func.get_basic_block_mut(func.last_block_idx()).terminator =
                Terminator::Return(Some(value), Some(stmt.span.clone()));
        }
        StmtKind::Block(block_stmt) => {
            let curr_block_idx = func.last_block_idx();

            func.get_basic_block_mut(curr_block_idx).terminator = Terminator::Branch {
                target: curr_block_idx + 1,
                span: None,
            };

            let _ = lower_block(
                func,
                block_stmt,
                Terminator::Branch {
                    target: curr_block_idx + 2,
                    span: None,
                },
            )?;
        }
        StmtKind::Expr(expr) => {
            let _ = lower_expr(func, expr)?;
        }
        StmtKind::If(if_stmt) => lower_if(func, if_stmt)?,
        StmtKind::While(while_stmt) => lower_while(func, while_stmt, stmt.span.clone())?,
        StmtKind::For(for_stmt) => lower_for(func, for_stmt, stmt.span.clone())?,
    }

    Ok(())
}

fn lower_declaration(
    func: &mut Function,
    stmt: &DeclStmt,
    span: &Span,
) -> Result<(), LoweringError> {
    match stmt {
        DeclStmt::Var(var_decl) => {
            lower_var_declaration(func, var_decl, span.clone())?;
        }
        DeclStmt::Struct(_struct_decl) => {
            return Err(LoweringError::UnsupportedFeature {
                feature: "struct declarations".to_string(),
                span: span.clone(),
            });
        }
        DeclStmt::Func(func_decl) => lower_inner_function(func, func_decl, span.clone())?,
    }

    Ok(())
}

pub fn lower_block(
    func: &mut Function,
    block: &BlockStmt,
    terminator: Terminator,
) -> Result<BlockId, LoweringError> {
    let old_sym_table = func.sym_table.clone();

    let block_id = func.add_block(terminator, Some(block.span.clone()));

    for s in block.stmts.iter() {
        lower_stmt(s, func)?;
    }

    func.sym_table = old_sym_table;

    Ok(block_id)
}
