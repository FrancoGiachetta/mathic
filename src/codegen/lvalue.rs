use melior::{
    dialect::{cf, func, llvm, ods},
    helpers::{BuiltinBlockExt, LlvmBlockExt},
    ir::{
        Block, BlockLike, Location,
        attribute::{DenseI32ArrayAttribute, FlatSymbolRefAttribute, TypeAttribute},
    },
};

use crate::{
    codegen::{MathicCodeGen, compiler_helper::CompilerHelper, function_ctx::FunctionCtx},
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
                let init_mlir_ty = self.get_compiled_type(fn_ctx.get_ir_func(), init.ty)?;
                let init_ty = self.get_type(fn_ctx.get_ir_func(), init.ty)?;
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
                let (mut ptr, mut ty_idx) =
                    fn_ctx.get_local(*local_idx).expect("invalid local idx");

                for m in modifier {
                    let ty = self.get_type(fn_ctx.get_ir_func(), ty_idx)?;
                    ptr = match m {
                        ValueModifier::Field(idx) => match ty {
                            MathicType::Adt { index, is_local } => {
                                let adt = if is_local {
                                    fn_ctx.get_ir_func().get_adt(index)
                                } else {
                                    self.ir.get_adt(index)
                                }
                                .ok_or(CodegenError::InvalidAdtIndex(index))?;

                                match adt {
                                    Adt::Struct(struct_adt) => {
                                        let field_ty_idx = struct_adt.fields[*idx].ty;
                                        let mem_ptr =
                                            block.append_op_result(llvm::get_element_ptr(
                                                self.ctx,
                                                ptr,
                                                DenseI32ArrayAttribute::new(
                                                    self.ctx,
                                                    &[0, *idx as i32],
                                                ),
                                                self.get_compiled_type(
                                                    fn_ctx.get_ir_func(),
                                                    ty_idx,
                                                )?,
                                                llvm::r#type::pointer(self.ctx, 0),
                                                location,
                                            ))?;

                                        ty_idx = field_ty_idx;

                                        mem_ptr
                                    }
                                }
                            }
                            other => unreachable!("{}", other),
                        },
                        ValueModifier::Index(idx) => match ty {
                            MathicType::Array { inner_ty_idx, .. } => {
                                let (index_ptr, index_ty) = fn_ctx.get_local(*idx).expect("");
                                let index_val = block.load(
                                    self.ctx,
                                    location,
                                    index_ptr,
                                    self.get_compiled_type(fn_ctx.get_ir_func(), index_ty)?,
                                )?;
                                let elem_ptr = block.append_op_result(
                                    ods::llvm::getelementptr(
                                        self.ctx,
                                        llvm::r#type::pointer(self.ctx, 0),
                                        ptr,
                                        &[index_val],
                                        DenseI32ArrayAttribute::new(self.ctx, &[0, i32::MIN]),
                                        TypeAttribute::new(
                                            self.get_compiled_type(fn_ctx.get_ir_func(), ty_idx)?,
                                        ),
                                        location,
                                    )
                                    .into(),
                                )?;

                                ty_idx = inner_ty_idx;

                                elem_ptr
                            }
                            other => unreachable!("{other}"),
                        },
                    };
                }

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
