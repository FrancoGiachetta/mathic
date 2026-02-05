use melior::{
    helpers::ArithBlockExt,
    ir::{Block, Location, Value},
};

use crate::{
    codegen::{MathicCodeGen, error::CodegenError},
    parser::grammar::expression::{ExprStmt, PrimaryExpr},
};

impl<'this, 'ctx> MathicCodeGen<'this, 'ctx>
where
    'this: 'ctx,
{
    pub fn compile_expression(
        &'ctx self,
        block: &'this Block<'ctx>,
        expr: ExprStmt,
    ) -> Result<Value<'ctx, 'this>, CodegenError> {
        match expr {
            ExprStmt::Primary(primary_expr) => self.compile_primary(block, primary_expr),
            ExprStmt::Assign { name: _, value: _ } => {
                unimplemented!("Assignment not implemented");
            }
            ExprStmt::BinOp {
                lhs: _,
                op: _,
                rhs: _,
            } => {
                unimplemented!("Binary operation not implemented");
            }
            ExprStmt::Logical {
                lhs: _,
                op: _,
                rhs: _,
            } => {
                unimplemented!("Logical operation not implemented")
            }
            ExprStmt::Unary { op: _, rhs: _ } => unimplemented!("Unary operation not implemented"),
            ExprStmt::Call { calle: _, args: _ } => unimplemented!("Function call not implemented"),
            ExprStmt::Index { name: _, pos: _ } => unimplemented!("Indexing not implemented"),
        }
    }

    pub fn compile_primary(
        &'ctx self,
        block: &'this Block<'ctx>,
        expr: PrimaryExpr,
    ) -> Result<Value<'ctx, 'this>, CodegenError> {
        let location = Location::unknown(&self.ctx);

        match expr {
            PrimaryExpr::Ident(_token) => unimplemented!("Identifier lookup not implemented"),
            PrimaryExpr::Num(val) => {
                let parsed_val: u64 = val.parse()?;
                Ok(block.const_int(&self.ctx, location, parsed_val, 64)?)
            }
            PrimaryExpr::Str(_) => unimplemented!("String literals not implemented"),
            PrimaryExpr::Bool(val) => Ok(block.const_int(&self.ctx, location, val, 1)?),
        }
    }
}
