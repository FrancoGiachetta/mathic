use crate::{
    lowering::{Lowerer, ir::value::Value},
    parser::ast::expression::ExprStmt,
};

impl Lowerer {
    pub fn lower_expr(&self, _expr: &ExprStmt) -> Value {
        todo!()
    }
}
