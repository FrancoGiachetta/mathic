use melior::{
    dialect::{arith::CmpiPredicate, func},
    helpers::{ArithBlockExt, BuiltinBlockExt},
    ir::{
        Block, Location, Value, ValueLike, attribute::FlatSymbolRefAttribute, r#type::IntegerType,
    },
};

use crate::{
    codegen::{MathicCodeGen, error::CodegenError},
    parser::{
        ast::expression::{ExprStmt, PrimaryExpr},
        token::Token,
    },
};

impl MathicCodeGen<'_> {
    pub fn compile_expression<'ctx, 'func>(
        &'func self,
        block: &'func Block<'ctx>,
        expr: &ExprStmt,
    ) -> Result<Value<'ctx, 'func>, CodegenError>
    where
        'func: 'ctx,
    {
        match expr {
            ExprStmt::Primary(primary_expr) => self.compile_primary(block, primary_expr),
            ExprStmt::Group(expr) => self.compile_expression(block, expr),
            ExprStmt::Assign { name: _, value: _ } => {
                unimplemented!("Assignment not implemented");
            }
            ExprStmt::BinOp { lhs, op, rhs } => self.compile_binop(block, lhs, op, rhs),
            ExprStmt::Logical { lhs, op, rhs } => self.compile_logical(block, lhs, op, rhs),
            ExprStmt::Unary { op, rhs } => self.compile_unary(block, op, rhs),
            ExprStmt::Call { calle, args } => self.compile_call(block, calle, args),
            ExprStmt::Index { name: _, pos: _ } => unimplemented!("Indexing not implemented"),
        }
    }

    fn compile_logical<'ctx, 'func>(
        &'func self,
        block: &'func Block<'ctx>,
        lhs: &ExprStmt,
        op: &Token,
        rhs: &ExprStmt,
    ) -> Result<Value<'ctx, 'func>, CodegenError>
    where
        'func: 'ctx,
    {
        let location = Location::unknown(self.ctx);

        let lhs_val = self.compile_expression(block, lhs)?;
        let rhs_val = self.compile_expression(block, rhs)?;

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

    fn compile_binop<'ctx, 'func>(
        &'func self,
        block: &'func Block<'ctx>,
        lhs: &ExprStmt,
        op: &Token,
        rhs: &ExprStmt,
    ) -> Result<Value<'ctx, 'func>, CodegenError>
    where
        'func: 'ctx,
    {
        let location = Location::unknown(self.ctx);

        let lhs_val = self.compile_expression(block, lhs)?;
        let rhs_val = self.compile_expression(block, rhs)?;

        Ok(match op {
            Token::EqEq => {
                let val = block.cmpi(self.ctx, CmpiPredicate::Eq, lhs_val, rhs_val, location)?;
                block.extui(val, lhs_val.r#type(), location)?
            }
            Token::BangEq => {
                let val = block.cmpi(self.ctx, CmpiPredicate::Ne, lhs_val, rhs_val, location)?;
                block.extui(val, lhs_val.r#type(), location)?
            }
            Token::Less => {
                let val = block.cmpi(
                    self.ctx,
                    // For now only positive numbers.
                    if false {
                        CmpiPredicate::Slt
                    } else {
                        CmpiPredicate::Ult
                    },
                    lhs_val,
                    rhs_val,
                    location,
                )?;
                block.extui(val, lhs_val.r#type(), location)?
            }
            Token::EqLess => {
                let val = block.cmpi(
                    self.ctx,
                    if false {
                        CmpiPredicate::Sle
                    } else {
                        CmpiPredicate::Ule
                    },
                    lhs_val,
                    rhs_val,
                    location,
                )?;
                block.extui(val, lhs_val.r#type(), location)?
            }
            Token::Greater => {
                let val = block.cmpi(
                    self.ctx,
                    if false {
                        CmpiPredicate::Sgt
                    } else {
                        CmpiPredicate::Ugt
                    },
                    lhs_val,
                    rhs_val,
                    location,
                )?;
                block.extui(val, lhs_val.r#type(), location)?
            }
            Token::EqGrater => {
                let val = block.cmpi(
                    self.ctx,
                    if false {
                        CmpiPredicate::Sge
                    } else {
                        CmpiPredicate::Uge
                    },
                    lhs_val,
                    rhs_val,
                    location,
                )?;
                block.extui(val, lhs_val.r#type(), location)?
            }
            Token::Plus => block.addi(lhs_val, rhs_val, location)?,
            Token::Minus => block.subi(lhs_val, rhs_val, location)?,
            Token::Star => block.muli(lhs_val, rhs_val, location)?,
            Token::Slash => {
                if true {
                    block.divsi(lhs_val, rhs_val, location)?
                } else {
                    block.divui(lhs_val, rhs_val, location)?
                }
            }
            _ => {
                return Err(CodegenError::InvalidOperation(format!(
                    "expected binary operation operation, got: {:?}",
                    op
                )));
            }
        })
    }

    fn compile_unary<'func, 'ctx>(
        &'func self,
        block: &'func Block<'ctx>,
        op: &Token,
        rhs: &ExprStmt,
    ) -> Result<Value<'ctx, 'func>, CodegenError>
    where
        'func: 'ctx,
    {
        let location = Location::unknown(self.ctx);
        let rhs_val = self.compile_expression(block, rhs)?;

        Ok(match op {
            Token::Bang => {
                let k0 = block.const_int_from_type(self.ctx, location, 0, rhs_val.r#type())?;
                block.andi(k0, rhs_val, location)?
            }
            Token::Minus => {
                let k_neg_1 =
                    block.const_int_from_type(self.ctx, location, -1, rhs_val.r#type())?;
                block.muli(k_neg_1, rhs_val, location)?
            }
            _ => {
                return Err(CodegenError::InvalidOperation(format!(
                    "expected unary operation operation, got: {:?}",
                    op
                )));
            }
        })
    }

    fn compile_call<'ctx, 'func>(
        &'func self,
        block: &'func Block<'ctx>,
        calle: &str,
        args: &[ExprStmt],
    ) -> Result<Value<'ctx, 'func>, CodegenError>
    where
        'func: 'ctx,
    {
        let location = Location::unknown(self.ctx);
        let args = args
            .iter()
            .map(|arg| self.compile_expression(block, arg))
            .collect::<Result<Vec<Value>, _>>()?;

        Ok(block.append_op_result(func::call(
            self.ctx,
            FlatSymbolRefAttribute::new(self.ctx, &format!("mathic__{}", calle)),
            &args,
            &[IntegerType::new(self.ctx, 64).into()],
            location,
        ))?)
    }

    fn compile_primary<'ctx, 'func>(
        &'func self,
        block: &'func Block<'ctx>,
        expr: &PrimaryExpr,
    ) -> Result<Value<'ctx, 'func>, CodegenError>
    where
        'func: 'ctx,
    {
        let location = Location::unknown(self.ctx);

        match expr {
            PrimaryExpr::Ident(_token) => unimplemented!("Identifier lookup not implemented"),
            PrimaryExpr::Num(val) => {
                let parsed_val: u64 = val.parse()?;
                Ok(block.const_int(self.ctx, location, parsed_val, 64)?)
            }
            PrimaryExpr::Str(_) => unimplemented!("String literals not implemented"),
            PrimaryExpr::Bool(val) => Ok(block.const_int(self.ctx, location, *val as u8, 64)?),
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
    #[case("df main() { return (true and false) or true; }", 1)]
    #[case("df main() { return true and (false or true); }", 1)]
    #[case("df main() { return (false or false) and true; }", 0)]
    fn test_logical_operations(#[case] source: &str, #[case] expected: i64) {
        assert_eq!(compile_and_execute(source), expected);
    }

    #[rstest]
    #[case("df main() { return 2 + 3 * 4; }", 14)]
    #[case("df main() { return (2 + 3) * 4; }", 20)]
    #[case("df main() { return 10 - 2 * 3; }", 4)]
    #[case("df main() { return (10 - 2) * 3; }", 24)]
    fn test_arithmetic_precedence(#[case] source: &str, #[case] expected: i64) {
        assert_eq!(compile_and_execute(source), expected);
    }
}
