use melior::{
    dialect::{cf, func, llvm},
    helpers::{ArithBlockExt, BuiltinBlockExt, LlvmBlockExt},
    ir::{Block, BlockLike, Location, attribute::FlatSymbolRefAttribute, r#type::IntegerType},
};

use crate::{
    codegen::{
        MathicCodeGen,
        compiler_helper::{CompilerHelper, debugging::DebugUtils},
        function_ctx::FunctionCtx,
    },
    diagnostics::CodegenError,
    lowering::ir::{basic_block::Terminator, instruction::LValInstruct},
};

impl MathicCodeGen<'_> {
    pub fn compile_statement<'ctx, 'func>(
        &'func self,
        fn_ctx: &mut FunctionCtx<'ctx, 'func>,
        block: &'func Block<'ctx>,
        inst: &LValInstruct,
        helper: &mut CompilerHelper,
    ) -> Result<(), CodegenError>
    where
        'func: 'ctx,
    {
        match inst {
            LValInstruct::Let {
                local_idx: _,
                init,
                span,
            } => {
                let location = self.get_location(*span)?;

                let init_val = self.compile_rvalue(fn_ctx, block, init, helper)?;
                let ptr = block.alloca1(
                    self.ctx,
                    location,
                    init.ty.get_compiled_type(self.ctx),
                    init.ty.align(),
                )?;

                block.store(self.ctx, location, ptr, init_val)?;

                fn_ctx.define_local(ptr, init.ty.get_compiled_type(self.ctx));
            }
            LValInstruct::Assign {
                local_idx,
                value,
                span,
            } => {
                let location = self.get_location(*span)?;

                let val = self.compile_rvalue(fn_ctx, block, value, helper)?;
                let (ptr, _) = fn_ctx.get_local(*local_idx).expect("invalid local idx");

                block.store(self.ctx, location, ptr, val)?;
            }
        }

        Ok(())
    }

    pub fn compile_terminator<'ctx, 'func>(
        &'func self,
        fn_ctx: &mut FunctionCtx<'ctx, 'func>,
        block: &'func Block<'ctx>,
        terminator: &Terminator,
        helper: &mut CompilerHelper,
    ) -> Result<(), CodegenError>
    where
        'func: 'ctx,
    {
        match terminator {
            Terminator::Return(rval_instruct, span) => match rval_instruct {
                Some(rvalue) => {
                    let val = self.compile_rvalue(fn_ctx, block, rvalue, helper)?;

                    let debug = helper.get_or_insert(|| DebugUtils::new());

                    block.append_operation(func::r#return(&[val], self.get_location(*span)?))
                }
                None => block.append_operation(func::r#return(&[], self.get_location(*span)?)),
            },
            Terminator::Branch { target, span } => block.append_operation(cf::br(
                &fn_ctx.get_block(*target),
                &[],
                self.get_location(*span)?,
            )),
            Terminator::CondBranch {
                condition,
                true_block,
                false_block,
                span,
            } => {
                let cond_val = self.compile_rvalue(fn_ctx, block, condition, helper)?;

                block.append_operation(cf::cond_br(
                    self.ctx,
                    cond_val,
                    &fn_ctx.get_block(*true_block),
                    &fn_ctx.get_block(*false_block),
                    &[],
                    &[],
                    self.get_location(*span)?,
                ))
            }
            Terminator::Unreachable(span) => {
                block.append_operation(llvm::unreachable(self.get_location(*span)?))
            }
            Terminator::Call {
                callee,
                args,
                return_dest: _,
                dest_block,
                return_ty,
                span,
            } => {
                let unknown_location = Location::unknown(self.ctx);

                let mut args_vals = Vec::with_capacity(args.len());
                for arg in args.iter() {
                    args_vals.push(self.compile_rvalue(fn_ctx, block, arg, helper)?);
                }

                let mlir_return_ty = return_ty.get_compiled_type(self.ctx);
                let return_ptr = block.alloca1(
                    self.ctx,
                    unknown_location,
                    mlir_return_ty,
                    return_ty.align(),
                )?;
                let return_value = block.append_op_result(func::call(
                    self.ctx,
                    FlatSymbolRefAttribute::new(self.ctx, &format!("mathic__{}", callee)),
                    &args_vals,
                    &[mlir_return_ty],
                    self.get_location(*span)?,
                ))?;

                block.store(self.ctx, unknown_location, return_ptr, return_value)?;

                fn_ctx.define_local(return_ptr, IntegerType::new(self.ctx, 64).into());

                block.append_operation(cf::br(
                    &fn_ctx.get_block(*dest_block),
                    &[],
                    self.get_location(None)?,
                ))
            }
        };

        Ok(())
    }

    pub fn compile_block<'ctx, 'func>(
        &'func self,
        fn_ctx: &mut FunctionCtx<'ctx, 'func>,
        block: &'func Block<'ctx>,
        stmts: &[LValInstruct],
        helper: &mut CompilerHelper,
    ) -> Result<(), CodegenError>
    where
        'func: 'ctx,
    {
        for stmt in stmts {
            self.compile_statement(fn_ctx, block, stmt, helper)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::compile_and_execute;
    use rstest::*;

    #[rstest]
    #[case("df main() i64 { return 0; }", 0)]
    #[case("df main() i64 { return 42; }", 42)]
    fn test_return_statements(#[case] source: &str, #[case] expected: i64) {
        assert_eq!(compile_and_execute(source), expected);
    }
}
