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
        Span,
        declaration::{FuncDecl, VarDecl},
    },
};

impl Lowerer {
    pub fn lower_var_declaration(&self, func: &mut Function, stmt: &VarDecl, span: Span) {
        let local_idx = func.add_local(Some(stmt.name.clone()), LocalKind::Temp);

        let init = self.lower_expr(func, &stmt.expr);

        func.push_instruction(LValInstruct::Let {
            local_idx,
            init,
            span: Some(span),
        });
    }

    pub fn lower_function(&self, func: &mut Function, stmt: &FuncDecl, span: Span) {
        let mut ir_func = Function::new(stmt.name.clone(), span);

        for param in stmt.params.iter() {
            ir_func.add_local(Some(param.name.clone()), LocalKind::Param);
        }

        for stmt in stmt.body.iter() {
            self.lower_stmt(stmt, &mut ir_func);
        }

        func.add_function(ir_func);
    }
}
