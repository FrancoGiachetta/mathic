use melior::{
    dialect::{cf, func, llvm},
    helpers::{BuiltinBlockExt, LlvmBlockExt},
    ir::{Block, BlockLike, BlockRef, Location, attribute::FlatSymbolRefAttribute},
};

use crate::{
    codegen::{
        MathicCodeGen, compiler_helper::CompilerHelper, dialect_integration::symbolic,
        function_ctx::FunctionCtx,
    },
    diagnostics::CodegenError,
    lowering::ir::basic_block::Terminator,
};

impl MathicCodeGen<'_> {
    pub fn compile_terminator<'ctx, 'func>(
        &'func self,
        fn_ctx: &mut FunctionCtx<'func>,
        mlir_blocks: &[BlockRef<'ctx, 'func>],
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

                    block.append_operation(func::r#return(&[val], self.get_location(*span)?))
                }
                None => block.append_operation(func::r#return(&[], self.get_location(*span)?)),
            },
            Terminator::Branch {
                target,
                block_args,
                span,
            } => {
                let block_args = block_args
                    .iter()
                    .map(|local_idx| fn_ctx.get_local(*local_idx).expect("invalid local idx").0)
                    .collect::<Vec<_>>();

                block.append_operation(cf::br(
                    &mlir_blocks[*target],
                    &block_args,
                    self.get_location(*span)?,
                ))
            }
            Terminator::CondBranch {
                condition,
                true_block,
                false_block,
                true_block_args,
                false_block_args,
                span,
                ..
            } => {
                let cond_val = self.compile_rvalue(fn_ctx, block, condition, helper)?;
                let true_block_args = true_block_args
                    .iter()
                    .map(|local_idx| fn_ctx.get_local(*local_idx).expect("invalid local idx").0)
                    .collect::<Vec<_>>();
                let false_block_args = false_block_args
                    .iter()
                    .map(|local_idx| fn_ctx.get_local(*local_idx).expect("invalid local idx").0)
                    .collect::<Vec<_>>();

                block.append_operation(cf::cond_br(
                    self.ctx,
                    cond_val,
                    &mlir_blocks[*true_block],
                    &mlir_blocks[*false_block],
                    &true_block_args,
                    &false_block_args,
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
                return_ty: return_ty_idx,
                span,
            } => {
                let unknown_location = Location::unknown(self.ctx);

                let mut args_vals = Vec::with_capacity(args.len());
                for arg in args.iter() {
                    args_vals.push(self.compile_rvalue(fn_ctx, block, arg, helper)?);
                }

                let mlir_return_ty =
                    self.get_compiled_type(fn_ctx.get_ir_func(), *return_ty_idx)?;
                let return_ty = self.get_type(fn_ctx.get_ir_func(), *return_ty_idx)?;
                let return_ptr = block.alloca1(
                    self.ctx,
                    unknown_location,
                    mlir_return_ty,
                    return_ty.align(self.ir, fn_ctx.get_ir_func()),
                )?;
                let return_value = block.append_op_result(func::call(
                    self.ctx,
                    FlatSymbolRefAttribute::new(self.ctx, &format!("mathic__{}", callee)),
                    &args_vals,
                    &[mlir_return_ty],
                    self.get_location(*span)?,
                ))?;

                block.store(self.ctx, unknown_location, return_ptr, return_value)?;

                fn_ctx.define_local(return_ptr, *return_ty_idx);

                block.append_operation(cf::br(
                    &mlir_blocks[*dest_block],
                    &[],
                    self.get_location(None)?,
                ))
            }
            Terminator::Eval {
                expr,
                sym_name,
                value,
                span,
                return_dest: _,
                return_ty_idx,
                dest_block,
            } => {
                let unknown_location = self.get_location(*span)?;

                let mlir_return_ty =
                    self.get_compiled_type(fn_ctx.get_ir_func(), *return_ty_idx)?;
                let return_ty = self.get_type(fn_ctx.get_ir_func(), *return_ty_idx)?;

                let expr = self.compile_rvalue(fn_ctx, block, expr, helper)?;
                let value = self.compile_rvalue(fn_ctx, block, value, helper)?;
                let return_value = block.append_op_result(symbolic::operation::eval(
                    self.ctx,
                    unknown_location,
                    expr,
                    sym_name,
                    value,
                ))?;

                let return_ptr = block.alloca1(
                    self.ctx,
                    unknown_location,
                    mlir_return_ty,
                    return_ty.align(self.ir, fn_ctx.get_ir_func()),
                )?;

                block.store(self.ctx, unknown_location, return_ptr, return_value)?;

                fn_ctx.define_local(return_ptr, *return_ty_idx);

                block.append_operation(cf::br(
                    &mlir_blocks[*dest_block],
                    &[],
                    self.get_location(None)?,
                ))
            }
        };

        Ok(())
    }
}
