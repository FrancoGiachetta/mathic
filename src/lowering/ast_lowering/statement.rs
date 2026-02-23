use crate::{
    lowering::{
        Lowerer,
        error::LoweringError,
        ir::{
            basic_block::{BlockId, Terminator},
            function::Function,
        },
    },
    parser::ast::{
        Span,
        declaration::DeclStmt,
        statement::{BlockStmt, Stmt, StmtKind},
    },
};

impl Lowerer {
    pub fn lower_stmt(&self, stmt: &Stmt, func: &mut Function) -> Result<(), LoweringError> {
        match &stmt.kind {
            StmtKind::Decl(decl) => self.lower_declaration(func, decl, &stmt.span)?,
            StmtKind::Return(expr) => {
                let value = self.lower_expr(func, expr)?;
                func.get_basic_block_mut(func.last_block_idx()).terminator =
                    Terminator::Return(Some(value), Some(stmt.span.clone()));
            }
            StmtKind::Block(block_stmt) => {
                let _ = self.lower_block(
                    func,
                    block_stmt,
                    Terminator::Branch {
                        target: func.last_block_idx() + 2,
                        span: None,
                    },
                )?;
            }
            StmtKind::Expr(expr) => {
                let _ = self.lower_expr(func, expr)?;
            }
            StmtKind::If(if_stmt) => self.lower_if(func, if_stmt)?,
            StmtKind::While(while_stmt) => self.lower_while(func, while_stmt, stmt.span.clone())?,
            StmtKind::For(for_stmt) => self.lower_for(func, for_stmt, stmt.span.clone())?,
        }
        Ok(())
    }

    fn lower_declaration(
        &self,
        func: &mut Function,
        stmt: &DeclStmt,
        span: &Span,
    ) -> Result<(), LoweringError> {
        match stmt {
            DeclStmt::Var(var_decl) => {
                self.lower_var_declaration(func, var_decl, span.clone())?;
            }
            DeclStmt::Struct(_struct_decl) => {
                return Err(LoweringError::UnsupportedFeature {
                    feature: "struct declarations".to_string(),
                    span: span.clone(),
                });
            }
            DeclStmt::Func(func_decl) => self.lower_function(func, func_decl, span.clone())?,
        }
        Ok(())
    }

    pub fn lower_block(
        &self,
        func: &mut Function,
        block: &BlockStmt,
        terminator: Terminator,
    ) -> Result<BlockId, LoweringError> {
        let old_sym_table = func.sym_table.clone();

        let block_id = func.add_block(terminator, Some(block.span.clone()));

        for s in block.stmts.iter() {
            self.lower_stmt(s, func)?;
        }

        func.sym_table = old_sym_table;

        Ok(block_id)
    }
}
