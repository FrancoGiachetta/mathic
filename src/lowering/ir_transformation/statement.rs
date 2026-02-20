use crate::{
    lowering::{
        Lowerer,
        ir::{
            basic_block::Terminator,
            function::{Function, LocalKind},
            instruction::LValInstruct,
        },
    },
    parser::ast::{
        declaration::DeclStmt,
        statement::{Stmt, StmtKind},
    },
};

impl Lowerer {
    pub fn lower_stmt(&self, stmt: &Stmt, func: &mut Function) {
        match &stmt.kind {
            StmtKind::Decl(DeclStmt::Var(var)) => {
                let local_idx = func.add_local(var.name.clone(), LocalKind::Temp);
                let init = self.lower_expr(func, &var.expr);
                func.push_instruction(LValInstruct::Let {
                    local_idx,
                    init,
                    span: Some(stmt.span.clone()),
                });
            }
            StmtKind::Decl(_) => {
                todo!()
            }
            StmtKind::Return(expr) => {
                let value = self.lower_expr(func, expr);
                func.last_basic_block().terminator =
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
