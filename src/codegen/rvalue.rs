use melior::{
    dialect::arith::CmpiPredicate,
    helpers::{ArithBlockExt, LlvmBlockExt},
    ir::{Block, Value, ValueLike, r#type::IntegerType},
};

use crate::{
    codegen::{MathicCodeGen, error::CodegenError, function_ctx::FunctionCtx},
    lowering::ir::{instruction::RValInstruct, value::Value as IRValue},
    parser::ast::{
        Span,
        expression::{ArithOp, BinaryOp, CmpOp, LogicalOp, UnaryOp},
    },
};

impl MathicCodeGen<'_> {
    pub fn compile_rvalue<'ctx, 'func>(
        &'func self,
        fn_ctx: &mut FunctionCtx<'ctx, 'func>,
        block: &'func Block<'ctx>,
        rvalue: &RValInstruct,
    ) -> Result<Value<'ctx, 'func>, CodegenError>
    where
        'func: 'ctx,
    {
        match rvalue {
            RValInstruct::Use(value, _) => self.compile_value_use(fn_ctx, block, value),
            RValInstruct::Binary { op, lhs, rhs, span } => {
                self.compile_binop(fn_ctx, block, lhs, *op, rhs, span.clone())
            }
            RValInstruct::Unary { op, rhs, span } => {
                self.compile_unary(fn_ctx, block, *op, rhs, span.clone())
            }
            RValInstruct::Logical { op, lhs, rhs, span } => {
                self.compile_logical(fn_ctx, block, lhs, *op, rhs, span.clone())
            }
        }
    }

    fn compile_logical<'ctx, 'func>(
        &'func self,
        fn_ctx: &mut FunctionCtx<'ctx, 'func>,
        block: &'func Block<'ctx>,
        lhs: &RValInstruct,
        op: LogicalOp,
        rhs: &RValInstruct,
        span: Span,
    ) -> Result<Value<'ctx, 'func>, CodegenError>
    where
        'func: 'ctx,
    {
        let location = self.get_location(Some(span))?;

        let lhs_val = self.compile_rvalue(fn_ctx, block, lhs)?;
        let rhs_val = self.compile_rvalue(fn_ctx, block, rhs)?;

        Ok(match op {
            LogicalOp::And => block.andi(lhs_val, rhs_val, location)?,
            LogicalOp::Or => block.ori(lhs_val, rhs_val, location)?,
        })
    }

    fn compile_binop<'ctx, 'func>(
        &'func self,
        fn_ctx: &mut FunctionCtx<'ctx, 'func>,
        block: &'func Block<'ctx>,
        lhs: &RValInstruct,
        op: BinaryOp,
        rhs: &RValInstruct,
        span: Span,
    ) -> Result<Value<'ctx, 'func>, CodegenError>
    where
        'func: 'ctx,
    {
        let location = self.get_location(Some(span))?;

        let lhs_val = self.compile_rvalue(fn_ctx, block, lhs)?;
        let rhs_val = self.compile_rvalue(fn_ctx, block, rhs)?;

        Ok(match op {
            BinaryOp::Compare(cmp) => match cmp {
                CmpOp::Eq => block.cmpi(self.ctx, CmpiPredicate::Eq, lhs_val, rhs_val, location)?,
                CmpOp::Ne => block.cmpi(self.ctx, CmpiPredicate::Ne, lhs_val, rhs_val, location)?,
                CmpOp::Lt => {
                    block.cmpi(
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
                    )?
                }
                CmpOp::Le => block.cmpi(
                    self.ctx,
                    if false {
                        CmpiPredicate::Sle
                    } else {
                        CmpiPredicate::Ule
                    },
                    lhs_val,
                    rhs_val,
                    location,
                )?,
                CmpOp::Gt => block.cmpi(
                    self.ctx,
                    if false {
                        CmpiPredicate::Sgt
                    } else {
                        CmpiPredicate::Ugt
                    },
                    lhs_val,
                    rhs_val,
                    location,
                )?,
                CmpOp::Ge => block.cmpi(
                    self.ctx,
                    if false {
                        CmpiPredicate::Sge
                    } else {
                        CmpiPredicate::Uge
                    },
                    lhs_val,
                    rhs_val,
                    location,
                )?,
            },
            BinaryOp::Arithmetic(arith) => match arith {
                ArithOp::Add => block.addi(lhs_val, rhs_val, location)?,
                ArithOp::Sub => block.subi(lhs_val, rhs_val, location)?,
                ArithOp::Mul => block.muli(lhs_val, rhs_val, location)?,
                ArithOp::Div => {
                    if true {
                        block.divsi(lhs_val, rhs_val, location)?
                    } else {
                        block.divui(lhs_val, rhs_val, location)?
                    }
                }

                ArithOp::Mod => todo!(),
            },
        })
    }

    fn compile_unary<'func, 'ctx>(
        &'func self,
        fn_ctx: &mut FunctionCtx<'ctx, 'func>,
        block: &'func Block<'ctx>,
        op: UnaryOp,
        rhs: &RValInstruct,
        span: Span,
    ) -> Result<Value<'ctx, 'func>, CodegenError>
    where
        'func: 'ctx,
    {
        let location = self.get_location(Some(span))?;
        let rhs_val = self.compile_rvalue(fn_ctx, block, rhs)?;

        Ok(match op {
            UnaryOp::Not => {
                let k0 = block.const_int_from_type(self.ctx, location, 0, rhs_val.r#type())?;
                block.andi(k0, rhs_val, location)?
            }
            UnaryOp::Neg => {
                let k_neg_1 =
                    block.const_int_from_type(self.ctx, location, -1, rhs_val.r#type())?;
                block.muli(k_neg_1, rhs_val, location)?
            }
        })
    }

    fn compile_value_use<'ctx, 'func>(
        &'func self,
        fn_ctx: &mut FunctionCtx<'ctx, 'func>,
        block: &'func Block<'ctx>,
        value: &IRValue,
    ) -> Result<Value<'ctx, 'func>, CodegenError>
    where
        'func: 'ctx,
    {
        let location = self.get_location(None)?;

        Ok(match value {
            IRValue::InMemory(local_idx) => {
                let local_ptr = fn_ctx.get_local(*local_idx).expect("Invalid local idx");

                block.load(
                    self.ctx,
                    location,
                    local_ptr,
                    IntegerType::new(self.ctx, 64).into(),
                )?
            }
            IRValue::Const(const_expr) => match const_expr {
                crate::lowering::ir::value::ContExpr::Int(val) => {
                    block.const_int(self.ctx, location, val, 64)?
                }
                crate::lowering::ir::value::ContExpr::Bool(val) => {
                    block.const_int(self.ctx, location, *val as u8, 1)?
                }
                crate::lowering::ir::value::ContExpr::Void => todo!(),
            },
        })
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
