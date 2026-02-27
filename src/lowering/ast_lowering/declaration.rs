use crate::{
    lowering::{
        Lowerer,
        error::LoweringError,
        ir::{
            function::{Function, LocalKind},
            instruction::LValInstruct,
        },
    },
    parser::ast::{
        Span,
        declaration::{FuncDecl, VarDecl},
    },
};

impl Lowerer {
    pub fn lower_var_declaration(
        &self,
        func: &mut Function,
        stmt: &VarDecl,
        span: Span,
    ) -> Result<(), LoweringError> {
        let init = self.lower_expr(func, &stmt.expr)?;
        let local_idx =
            func.add_local(Some(stmt.name.clone()), Some(span.clone()), LocalKind::Temp)?;

        // FUTURE: check the expression is the same type as the declaration.

        func.push_instruction(LValInstruct::Let {
            local_idx,
            init,
            span: Some(span),
        });

        Ok(())
    }

    pub fn lower_inner_function(
        &self,
        func: &mut Function,
        stmt: &FuncDecl,
        span: Span,
    ) -> Result<(), LoweringError> {
        let mut inner_func = Function::new(stmt.name.clone(), span);

        for param in stmt.params.iter() {
            inner_func.add_local(
                Some(param.name.clone()),
                Some(param.span.clone()),
                LocalKind::Param,
            )?;
        }

        for stmt in stmt.body.iter() {
            self.lower_stmt(stmt, &mut inner_func)?;
        }

        func.add_function(inner_func);

        Ok(())
    }
}
