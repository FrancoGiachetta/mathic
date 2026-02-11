use melior::{
    Context,
    dialect::arith::CmpiPredicate,
    helpers::ArithBlockExt,
    ir::{Block, Location, Value, r#type::IntegerType},
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
            ExprStmt::BinOp { lhs, op, rhs } => self.compile_binop(ctx, block, *lhs, op, *rhs),
            ExprStmt::Logical { lhs, op, rhs } => self.compile_logical(ctx, block, *lhs, op, *rhs),
            ExprStmt::Unary { op: _, rhs: _ } => unimplemented!("Unary operation not implemented"),
            ExprStmt::Call { calle: _, args: _ } => unimplemented!("Function call not implemented"),
            ExprStmt::Index { name: _, pos: _ } => unimplemented!("Indexing not implemented"),
        }
    }

    fn compile_binop(
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

        let val = match op {
            Token::EqEq => block.cmpi(ctx, CmpiPredicate::Eq, lhs_val, rhs_val, location)?,
            Token::BangEq => block.cmpi(ctx, CmpiPredicate::Ne, lhs_val, rhs_val, location)?,
            Token::Less => block.cmpi(
                ctx,
                // For now only positive numbers.
                if false {
                    CmpiPredicate::Slt
                } else {
                    CmpiPredicate::Ult
                },
                lhs_val,
                rhs_val,
                location,
            )?,
            Token::EqLess => block.cmpi(
                ctx,
                if false {
                    CmpiPredicate::Sle
                } else {
                    CmpiPredicate::Ule
                },
                lhs_val,
                rhs_val,
                location,
            )?,
            Token::Greater => block.cmpi(
                ctx,
                if false {
                    CmpiPredicate::Sgt
                } else {
                    CmpiPredicate::Ugt
                },
                lhs_val,
                rhs_val,
                location,
            )?,
            Token::EqGrater => block.cmpi(
                ctx,
                if false {
                    CmpiPredicate::Sge
                } else {
                    CmpiPredicate::Uge
                },
                lhs_val,
                rhs_val,
                location,
            )?,
            _ => {
                return Err(CodegenError::InvalidOperation(format!(
                    "expected binary operation operation, got: {:?}",
                    op
                )));
            }
        };

        Ok(block.extui(val, IntegerType::new(ctx, 64).into(), location)?)
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

#[cfg(test)]
mod tests {
    use crate::test_utils::compile_and_execute;
    use rstest::*;

    #[rstest]
    #[case("df main() { return 42 == 42; }", 1)]
    #[case("df main() { return 42 != 21; }", 1)]
    #[case("df main() { return 42 == 21; }", 0)]
    #[case("df main() { return 42 > 21; }", 1)]
    #[case("df main() { return 21 < 42; }", 1)]
    #[case("df main() { return 42 >= 42; }", 1)]
    #[case("df main() { return 21 <= 42; }", 1)]
    fn test_binary_operations(#[case] source: &str, #[case] expected: i64) {
        assert_eq!(compile_and_execute(source), expected);
    }

    #[rstest]
    #[case("df main() { return true and true; }", 1)]
    #[case("df main() { return false and true; }", 0)]
    #[case("df main() { return true or true; }", 1)]
    #[case("df main() { return true or false; }", 1)]
    #[case("df main() { return false or false; }", 0)]
    fn test_logical_operations(#[case] source: &str, #[case] expected: i64) {
        assert_eq!(compile_and_execute(source), expected);
    }
}
