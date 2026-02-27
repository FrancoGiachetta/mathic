use melior::{
    dialect::{cf, func, llvm},
    helpers::{BuiltinBlockExt, LlvmBlockExt},
    ir::{Block, BlockLike, Location, attribute::FlatSymbolRefAttribute, r#type::IntegerType},
};

use crate::{
    codegen::{MathicCodeGen, function_ctx::FunctionCtx},
    diagnostics::CodegenError,
    lowering::ir::{basic_block::Terminator, instruction::LValInstruct},
};

impl MathicCodeGen<'_> {
    pub fn compile_statement<'ctx, 'func>(
        &'func self,
        fn_ctx: &mut FunctionCtx<'ctx, 'func>,
        block: &'func Block<'ctx>,
        inst: &LValInstruct,
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
                let location = self.get_location(span.clone())?;

                let init_val = self.compile_rvalue(fn_ctx, block, init)?;
                let ptr =
                    block.alloca1(self.ctx, location, IntegerType::new(self.ctx, 64).into(), 8)?;

                block.store(self.ctx, location, ptr, init_val)?;

                fn_ctx.define_local(ptr);
            }
            LValInstruct::Assign {
                local_idx,
                value,
                span,
            } => {
                let location = self.get_location(span.clone())?;

                let val = self.compile_rvalue(fn_ctx, block, value)?;
                let ptr = fn_ctx.get_local(*local_idx).expect("invalid local idx");

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
    ) -> Result<(), CodegenError>
    where
        'func: 'ctx,
    {
        match terminator {
            Terminator::Return(rval_instruct, span) => match rval_instruct {
                Some(rvalue) => {
                    let val = self.compile_rvalue(fn_ctx, block, rvalue)?;

                    block.append_operation(func::r#return(&[val], self.get_location(span.clone())?))
                }
                None => {
                    block.append_operation(func::r#return(&[], self.get_location(span.clone())?))
                }
            },
            Terminator::Branch { target, span } => block.append_operation(cf::br(
                &fn_ctx.get_block(*target),
                &[],
                self.get_location(span.clone())?,
            )),
            Terminator::CondBranch {
                condition,
                true_block,
                false_block,
                span,
            } => {
                let cond_val = self.compile_rvalue(fn_ctx, block, condition)?;

                block.append_operation(cf::cond_br(
                    self.ctx,
                    cond_val,
                    &fn_ctx.get_block(*true_block),
                    &fn_ctx.get_block(*false_block),
                    &[],
                    &[],
                    self.get_location(span.clone())?,
                ))
            }
            Terminator::Unreachable(span) => {
                block.append_operation(llvm::unreachable(self.get_location(span.clone())?))
            }
            Terminator::Call {
                callee,
                args,
                return_dest: _,
                dest_block,
                span,
            } => {
                let unknown_location = Location::unknown(self.ctx);

                let mut args_vals = Vec::with_capacity(args.len());
                for arg in args.iter() {
                    args_vals.push(self.compile_rvalue(fn_ctx, block, arg)?);
                }

                let return_ptr = block.alloca1(
                    self.ctx,
                    unknown_location,
                    IntegerType::new(self.ctx, 64).into(),
                    8,
                )?;

                let return_value = block.append_op_result(func::call(
                    self.ctx,
                    FlatSymbolRefAttribute::new(self.ctx, &format!("mathic__{}", callee)),
                    &args_vals,
                    &[IntegerType::new(self.ctx, 64).into()],
                    self.get_location(span.clone())?,
                ))?;

                block.store(self.ctx, unknown_location, return_ptr, return_value)?;

                fn_ctx.define_local(return_ptr);

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
    ) -> Result<(), CodegenError>
    where
        'func: 'ctx,
    {
        for stmt in stmts {
            self.compile_statement(fn_ctx, block, stmt)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::compile_and_execute;
    use rstest::*;

    #[rstest]
    #[case("df main() { return 0; }", 0)]
    #[case("df main() { return 42; }", 42)]
    fn test_return_statements(#[case] source: &str, #[case] expected: i64) {
        assert_eq!(compile_and_execute(source), expected);
    }
}
