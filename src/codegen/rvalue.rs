use melior::{
    dialect::{arith::CmpiPredicate, llvm, ods},
    helpers::{ArithBlockExt, BuiltinBlockExt, LlvmBlockExt},
    ir::{Block, Value, ValueLike, attribute::StringAttribute, r#type::IntegerType},
};

use crate::{
    codegen::{MathicCodeGen, compiler_helper::CompilerHelper, function_ctx::FunctionCtx},
    diagnostics::CodegenError,
    lowering::ir::{
        instruction::{RValInstruct, RValueKind},
        value::{ConstExpr, NumericConst, Value as IRValue},
    },
    parser::{
        Span,
        ast::expression::{ArithOp, BinaryOp, CmpOp, LogicalOp, UnaryOp},
    },
};

impl MathicCodeGen<'_> {
    pub fn compile_rvalue<'ctx, 'func>(
        &'func self,
        fn_ctx: &mut FunctionCtx<'ctx, 'func>,
        block: &'func Block<'ctx>,
        rvalue: &RValInstruct,
        helper: &mut CompilerHelper,
    ) -> Result<Value<'ctx, 'func>, CodegenError>
    where
        'func: 'ctx,
    {
        match &rvalue.kind {
            RValueKind::Use { value, .. } => self.compile_value_use(fn_ctx, block, value, helper),
            RValueKind::Binary {
                op, lhs, rhs, span, ..
            } => self.compile_binop(fn_ctx, block, lhs, *op, rhs, *span, helper),
            RValueKind::Unary { op, rhs, span, .. } => {
                self.compile_unary(fn_ctx, block, *op, rhs, *span, helper)
            }
            RValueKind::Logical {
                op, lhs, rhs, span, ..
            } => self.compile_logical(fn_ctx, block, lhs, *op, rhs, *span, helper),
            RValueKind::Init(_) => todo!(),
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn compile_logical<'ctx, 'func>(
        &'func self,
        fn_ctx: &mut FunctionCtx<'ctx, 'func>,
        block: &'func Block<'ctx>,
        lhs: &RValInstruct,
        op: LogicalOp,
        rhs: &RValInstruct,
        span: Span,
        helper: &mut CompilerHelper,
    ) -> Result<Value<'ctx, 'func>, CodegenError>
    where
        'func: 'ctx,
    {
        let location = self.get_location(Some(span))?;

        let lhs_val = self.compile_rvalue(fn_ctx, block, lhs, helper)?;
        let rhs_val = self.compile_rvalue(fn_ctx, block, rhs, helper)?;

        Ok(match op {
            LogicalOp::And => block.andi(lhs_val, rhs_val, location)?,
            LogicalOp::Or => block.ori(lhs_val, rhs_val, location)?,
        })
    }

    #[allow(clippy::too_many_arguments)]
    fn compile_binop<'ctx, 'func>(
        &'func self,
        fn_ctx: &mut FunctionCtx<'ctx, 'func>,
        block: &'func Block<'ctx>,
        lhs: &RValInstruct,
        op: BinaryOp,
        rhs: &RValInstruct,
        span: Span,
        helper: &mut CompilerHelper,
    ) -> Result<Value<'ctx, 'func>, CodegenError>
    where
        'func: 'ctx,
    {
        let location = self.get_location(Some(span))?;

        let lhs_val = self.compile_rvalue(fn_ctx, block, lhs, helper)?;
        let rhs_val = self.compile_rvalue(fn_ctx, block, rhs, helper)?;

        Ok(match op {
            BinaryOp::Compare(cmp) => match cmp {
                CmpOp::Eq => block.cmpi(self.ctx, CmpiPredicate::Eq, lhs_val, rhs_val, location)?,
                CmpOp::Ne => block.cmpi(self.ctx, CmpiPredicate::Ne, lhs_val, rhs_val, location)?,
                CmpOp::Lt => block.cmpi(
                    self.ctx,
                    if lhs.ty.is_signed() {
                        CmpiPredicate::Slt
                    } else {
                        CmpiPredicate::Ult
                    },
                    lhs_val,
                    rhs_val,
                    location,
                )?,
                CmpOp::Le => block.cmpi(
                    self.ctx,
                    if lhs.ty.is_signed() {
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
                    if lhs.ty.is_signed() {
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
                    if lhs.ty.is_signed() {
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
                    if lhs.ty.is_signed() {
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
        helper: &mut CompilerHelper,
    ) -> Result<Value<'ctx, 'func>, CodegenError>
    where
        'func: 'ctx,
    {
        let location = self.get_location(Some(span))?;
        let rhs_val = self.compile_rvalue(fn_ctx, block, rhs, helper)?;

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
        _helper: &mut CompilerHelper,
    ) -> Result<Value<'ctx, 'func>, CodegenError>
    where
        'func: 'ctx,
    {
        let location = self.get_location(None)?;

        Ok(match value {
            IRValue::InMemory(local_idx) => {
                let (local_ptr, local_ty) =
                    fn_ctx.get_local(*local_idx).expect("Invalid local idx");

                block.load(self.ctx, location, local_ptr, local_ty)?
            }
            IRValue::Const(const_expr) => match const_expr {
                ConstExpr::Numeric(num_const) => match num_const {
                    NumericConst::I8(val) => block.const_int_from_type(
                        self.ctx,
                        location,
                        val,
                        IntegerType::new(self.ctx, 8).into(),
                    )?,
                    NumericConst::I16(val) => block.const_int_from_type(
                        self.ctx,
                        location,
                        val,
                        IntegerType::new(self.ctx, 16).into(),
                    )?,
                    NumericConst::I32(val) => block.const_int_from_type(
                        self.ctx,
                        location,
                        val,
                        IntegerType::new(self.ctx, 32).into(),
                    )?,
                    NumericConst::I64(val) => block.const_int_from_type(
                        self.ctx,
                        location,
                        val,
                        IntegerType::new(self.ctx, 64).into(),
                    )?,
                    NumericConst::I128(val) => block.const_int_from_type(
                        self.ctx,
                        location,
                        val,
                        IntegerType::new(self.ctx, 128).into(),
                    )?,
                    NumericConst::U8(val) => block.const_int_from_type(
                        self.ctx,
                        location,
                        val,
                        IntegerType::new(self.ctx, 8).into(),
                    )?,
                    NumericConst::U16(val) => block.const_int_from_type(
                        self.ctx,
                        location,
                        val,
                        IntegerType::new(self.ctx, 16).into(),
                    )?,
                    NumericConst::U32(val) => block.const_int_from_type(
                        self.ctx,
                        location,
                        val,
                        IntegerType::new(self.ctx, 32).into(),
                    )?,
                    NumericConst::U64(val) => block.const_int_from_type(
                        self.ctx,
                        location,
                        val,
                        IntegerType::new(self.ctx, 64).into(),
                    )?,
                    NumericConst::U128(val) => block.const_int_from_type(
                        self.ctx,
                        location,
                        val,
                        IntegerType::new(self.ctx, 128).into(),
                    )?,
                    NumericConst::F32(_) => todo!(),
                    NumericConst::F64(_) => todo!(),
                },
                ConstExpr::Str(s) => {
                    // Str is a fixed size, null terminated array of bytes
                    // which is allocated in the stack.
                    let str_len_with_sentinel = s.len() as u32 + 1;

                    let u8_ty = IntegerType::new(self.ctx, 8).into();
                    let arr_ty = llvm::r#type::array(u8_ty, str_len_with_sentinel);

                    // Rust String does not hold a null byte at the end, so we need to add it.
                    let mut s_with_null = s.clone();
                    s_with_null.push('\0');

                    let str_const = block.append_op_result(
                        ods::llvm::mlir_constant(
                            self.ctx,
                            arr_ty,
                            StringAttribute::new(self.ctx, &s_with_null).into(),
                            location,
                        )
                        .into(),
                    )?;

                    let ptr = block.alloca1(self.ctx, location, arr_ty, 8)?;

                    block.store(self.ctx, location, ptr, str_const)?;

                    ptr
                }
                ConstExpr::Char(c) => block.const_int(self.ctx, location, *c, 8)?,
                ConstExpr::Bool(val) => block.const_int(self.ctx, location, *val as u8, 1)?,
                ConstExpr::Void => todo!(),
            },
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::compile_and_execute;
    use rstest::*;

    #[rstest]
    #[case("df main() i32 { return 2 + 3 * 4; }", 14)]
    #[case("df main() i32 { return (2 + 3) * 4; }", 20)]
    #[case("df main() i32 { return 10 - 2 * 3; }", 4)]
    #[case("df main() i32 { return (10 - 2) * 3; }", 24)]
    fn test_arithmetic_precedence(#[case] source: &str, #[case] expected: i64) {
        assert_eq!(compile_and_execute(source), expected);
    }
}
