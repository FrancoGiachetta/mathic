use melior::{
    Context,
    helpers::ArithBlockExt,
    ir::{Block, Location, Value},
};

use crate::{
    codegen::{MathicCodeGen, error::CodegenError},
    parser::{
        grammar::expression::{ExprStmt, PrimaryExpr},
        token::Token,
    },
};

impl<'this, 'ctx> MathicCodeGen<'this, 'ctx>
where
    'this: 'ctx,
{
    pub fn compile_expression(
        &self,
        ctx: &'ctx Context,
        block: &'this Block<'ctx>,
        expr: ExprStmt,
    ) -> Result<Value<'ctx, 'this>, CodegenError> {
        match expr {
            ExprStmt::Primary(primary_expr) => self.compile_primary(ctx, block, primary_expr),
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
            ExprStmt::Logical { lhs, op, rhs } => self.compile_logical(ctx, block, *lhs, op, *rhs),
            ExprStmt::Unary { op: _, rhs: _ } => unimplemented!("Unary operation not implemented"),
            ExprStmt::Call { calle: _, args: _ } => unimplemented!("Function call not implemented"),
            ExprStmt::Index { name: _, pos: _ } => unimplemented!("Indexing not implemented"),
        }
    }

    fn compile_logical(
        &self,
        ctx: &'ctx Context,
        block: &'this Block<'ctx>,
        lhs: ExprStmt,
        op: Token,
        rhs: ExprStmt,
    ) -> Result<Value<'ctx, 'this>, CodegenError> {
        let location = Location::unknown(ctx);

        let lhs_val = self.compile_expression(ctx, block, lhs)?;
        let rhs_val = self.compile_expression(ctx, block, rhs)?;

        Ok(match op {
            Token::And => block.andi(lhs_val, rhs_val, location)?,
            Token::Or => block.ori(lhs_val, rhs_val, location)?,
            _ => {
                return Err(CodegenError::InvalidOperation(format!(
                    "expected logical operation, got: {:?}",
                    op
                )));
            }
        })
    }

    fn compile_primary(
        &self,
        ctx: &'ctx Context,
        block: &'this Block<'ctx>,
        expr: PrimaryExpr,
    ) -> Result<Value<'ctx, 'this>, CodegenError> {
        let location = Location::unknown(ctx);

        match expr {
            PrimaryExpr::Ident(_token) => unimplemented!("Identifier lookup not implemented"),
            PrimaryExpr::Num(val) => {
                let parsed_val: u64 = val.parse()?;
                Ok(block.const_int(ctx, location, parsed_val, 64)?)
            }
            PrimaryExpr::Str(_) => unimplemented!("String literals not implemented"),
            PrimaryExpr::Bool(val) => Ok(block.const_int(ctx, location, val as u8, 64)?),
        }
    }
}
