use crate::parser::ast::{
    declaration::DeclStmt,
    statement::{Stmt, StmtKind},
};

use super::Lowerer;
use super::ir::basic_block::Terminator;
use super::ir::function::Function;
use super::ir::instruction::Instruction;

impl Lowerer {
    pub fn lower_stmt(&self, stmt: &Stmt, func: &mut Function) {
        match &stmt.kind {
            StmtKind::Decl(DeclStmt::Var(var)) => {
                let init = self.lower_expr(&var.expr);
                func.add_local(var.name.clone());
                func.entry_block.instructions.push(Instruction::Let {
                    name: var.name.clone(),
                    init,
                    span: Some(stmt.span.clone()),
                });
            }
            StmtKind::Decl(_) => {
                todo!()
            }
            StmtKind::Return(expr) => {
                let value = self.lower_expr(expr);
                func.entry_block.terminator =
                    Terminator::Return(Some(value), Some(stmt.span.clone()));
            }
            StmtKind::Block(block_stmt) => {
                for s in &block_stmt.stmts {
                    self.lower_stmt(s, func);
                }
            }
            StmtKind::If(_) | StmtKind::While(_) | StmtKind::For(_) | StmtKind::Expr(_) => {
                todo!()
            }
        }
    }
}
