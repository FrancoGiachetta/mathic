use std::mem;

use crate::{
    lowering::{
        Lowerer,
        ir::{basic_block::Terminator, function::Function},
    },
    parser::ast::{
        Span,
        declaration::DeclStmt,
        statement::{BlockStmt, Stmt, StmtKind},
    },
};

impl Lowerer {
    pub fn lower_stmt(&self, stmt: &Stmt, func: &mut Function) {
        match &stmt.kind {
            StmtKind::Decl(decl) => self.lower_declaration(func, decl, &stmt.span),
            StmtKind::Return(expr) => {
                let value = self.lower_expr(func, expr);
                func.get_basic_block_mut(func.last_block_idx()).terminator =
                    Terminator::Return(Some(value), Some(stmt.span.clone()));
            }
            StmtKind::Block(block_stmt) => self.lower_block(func, block_stmt),
            StmtKind::Expr(expr) => {
                let _ = self.lower_expr(func, expr);
            }
            StmtKind::If(if_stmt) => self.lower_if(func, if_stmt),
            StmtKind::While(_) | StmtKind::For(_) => {
                todo!()
            }
        }
    }

    fn lower_declaration(&self, func: &mut Function, stmt: &DeclStmt, span: &Span) {
        match stmt {
            DeclStmt::Var(var_decl) => {
                self.lower_var_declaration(func, var_decl, span.clone());
            }
            DeclStmt::Struct(_struct_decl) => todo!(),
            DeclStmt::Func(func_decl) => self.lower_function(func, func_decl, span.clone()),
        }
    }

    fn lower_block(&self, func: &mut Function, block: &BlockStmt) {
        // Create a new scope for the block.
        let old_sym_table = mem::take(&mut func.sym_table);
        let curr_block_idx = func.last_block_idx();

        func.get_basic_block_mut(curr_block_idx).terminator = Terminator::Branch {
            target: curr_block_idx,
            span: None,
        };

        func.add_block(
            Terminator::Branch {
                target: curr_block_idx + 2,
                span: None,
            },
            Some(block.span.clone()),
        );

        for s in block.stmts.iter() {
            self.lower_stmt(s, func);
        }

        // Once the
        func.sym_table = old_sym_table;
    }
}
