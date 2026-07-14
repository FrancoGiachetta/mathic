use melior::{
    dialect::{cf, func, llvm},
    helpers::{BuiltinBlockExt, GepIndex, LlvmBlockExt},
    ir::{Block, BlockLike, Location, attribute::FlatSymbolRefAttribute},
};

use crate::{
    codegen::{
        MathicCodeGen, compiler_helper::CompilerHelper, dialect_integration::symbolic,
        function_ctx::FunctionCtx,
    },
    diagnostics::CodegenError,
    lowering::ir::{
        adts::Adt, basic_block::Terminator, instruction::LValInstruct, types::MathicType,
        value::ValueModifier,
    },
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
                let init_ty = self.get_type(fn_ctx.get_ir_func(), init.ty)?;

                if init_ty.is_symbolic() {
                    fn_ctx.define_local(init_val, init.ty);
                    return Ok(());
                }

                let init_mlir_ty = self.get_compiled_type(fn_ctx.get_ir_func(), init.ty)?;
                let ptr = block.alloca1(
                    self.ctx,
                    location,
                    init_mlir_ty,
                    init_ty.align(self.ir, fn_ctx.get_ir_func()),
                )?;

                block.store(self.ctx, location, ptr, init_val)?;

                fn_ctx.define_local(ptr, init.ty);
            }
            LValInstruct::Assign {
                local_idx,
                value,
                modifier,
                span,
            } => {
                let location = self.get_location(*span)?;

                let val = self.compile_rvalue(fn_ctx, block, value, helper)?;
                let val_ty = self.get_type(fn_ctx.get_ir_func(), value.ty)?;

                if val_ty.is_symbolic() {
                    fn_ctx.assign_local(*local_idx, val);
                    return Ok(());
                }

                let (mut ptr, mut ty_idx) =
                    fn_ctx.get_local(*local_idx).expect("invalid local idx");

                for m in modifier {
                    ptr = match m {
                        ValueModifier::Field(idx) => match self
                            .get_type(fn_ctx.get_ir_func(), ty_idx)?
                        {
                            MathicType::Adt { index, is_local } => {
                                let adt = if is_local {
                                    fn_ctx.get_ir_func().get_adt(index)
                                } else {
                                    self.ir.get_adt(index)
                                }
                                .ok_or(CodegenError::InvalidAdtIndex(index))?;

                                match adt {
                                    Adt::Struct(struct_adt) => {
                                        let field_ty = struct_adt.fields[*idx].ty;
                                        ty_idx = field_ty;
                                        block.gep(
                                            self.ctx,
                                            location,
                                            ptr,
                                            &[GepIndex::Const(*idx as i32)],
                                            self.get_compiled_type(fn_ctx.get_ir_func(), ty_idx)?,
                                        )?
                                    }
                                }
                            }
                            _ => unreachable!(),
                        },
                    };
                }

                block.store(self.ctx, location, ptr, val)?;
            }
            LValInstruct::Sym {
                local_idx: _,
                sym_name,
                ty,
                span,
            } => {
                let func_ir = fn_ctx.get_ir_func();
                let location = self.get_location(*span)?;

                let sym = block.append_op_result(symbolic::operation::sym(
                    self.ctx,
                    location,
                    sym_name,
                    self.get_compiled_type(func_ir, *ty)?,
                ))?;

                fn_ctx.define_local(sym, *ty);
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

                    block.append_operation(func::r#return(&[val], self.get_location(*span)?))
                }
                None => block.append_operation(func::r#return(&[], self.get_location(*span)?)),
            },
            Terminator::Branch {
                target,
                successor_args,
                span,
            } => {
                let successor_args = successor_args
                    .iter()
                    .map(|local_idx| fn_ctx.get_local(*local_idx).expect("invalid local idx").0)
                    .collect::<Vec<_>>();

                block.append_operation(cf::br(
                    &fn_ctx.get_block(*target),
                    &successor_args,
                    self.get_location(*span)?,
                ))
            }
            Terminator::CondBranch {
                condition,
                true_block,
                false_block,
                true_successor_args,
                false_successor_args,
                span,
                ..
            } => {
                let cond_val = self.compile_rvalue(fn_ctx, block, condition, helper)?;
                let true_block_args = true_successor_args
                    .iter()
                    .map(|local_idx| fn_ctx.get_local(*local_idx).expect("invalid local idx").0)
                    .collect::<Vec<_>>();
                let false_block_args = false_successor_args
                    .iter()
                    .map(|local_idx| fn_ctx.get_local(*local_idx).expect("invalid local idx").0)
                    .collect::<Vec<_>>();

                block.append_operation(cf::cond_br(
                    self.ctx,
                    cond_val,
                    &fn_ctx.get_block(*true_block),
                    &fn_ctx.get_block(*false_block),
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
                    &fn_ctx.get_block(*dest_block),
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
