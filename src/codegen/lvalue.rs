use melior::{
    dialect::func,
    helpers::{BuiltinBlockExt, GepIndex, LlvmBlockExt},
    ir::{
        Attribute, Block, BlockLike, Identifier, Region,
        attribute::{StringAttribute, TypeAttribute},
        r#type::FunctionType,
    },
};

use crate::{
    codegen::{
        MathicCodeGen, compiler_helper::CompilerHelper, dialect_integration::symbolic,
        function_ctx::FunctionCtx,
    },
    diagnostics::CodegenError,
    lowering::ir::{
        adts::Adt, function::Function, instruction::LValInstruct, types::MathicType,
        value::ValueModifier,
    },
};

impl MathicCodeGen<'_> {
    pub fn compile_statement<'ctx, 'func>(
        &'func self,
        fn_ctx: &mut FunctionCtx<'func>,
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

    pub fn compile_function<'ctx, 'func>(
        &'func self,
        ir_func: &Function,
        attributes: &[(Identifier<'_>, Attribute<'_>)],
        helper: &mut CompilerHelper,
    ) -> Result<(), CodegenError>
    where
        'func: 'ctx,
    {
        let location = self.get_location(None)?;

        let return_ty = self.get_compiled_type(ir_func, ir_func.return_ty)?;
        let mut params_types = Vec::with_capacity(ir_func.params_tys.len());
        let mut entry_block_params = Vec::with_capacity(ir_func.params_tys.len());

        // Prepare the function's params' types and the entry block params as
        // well.
        for param_ty in ir_func.params_tys.iter() {
            let mlir_ty = self.get_compiled_type(ir_func, *param_ty)?;

            params_types.push(mlir_ty);
            entry_block_params.push((mlir_ty, location));
        }

        let region = Region::new();
        let (mut fn_ctx, mlir_blocks) =
            self.create_fn_ctx(&region, location, ir_func, &entry_block_params)?;

        // Precompile inner functions.
        for inner_func in ir_func.get_inner_functions() {
            self.compile_function(
                inner_func,
                &[(
                    Identifier::new(self.ctx, "sym_visibility"),
                    StringAttribute::new(self.ctx, "private").into(),
                )],
                helper,
            )?;
        }

        // Generate code for every basic_block. For each of them, we first
        // compile their instructions and their terminator instruction.
        for (block, mlir_block) in ir_func.basic_blocks.iter().zip(&mlir_blocks) {
            // Override locals with the block arg they should be pointing at.
            // Block args must always refer to a valid local value.
            for (idx, arg) in block.args.iter().enumerate() {
                let block_arg = mlir_block.arg(idx)?;
                if self.get_type(ir_func, arg.ty)?.is_symbolic() {
                    fn_ctx.assign_local(arg.local_idx, block_arg);
                } else {
                    mlir_block.store(
                        self.ctx,
                        location,
                        fn_ctx.get_local(arg.local_idx)?.0,
                        block_arg,
                    )?;
                }
            }

            self.compile_block(&mut fn_ctx, mlir_block, &block.instructions, helper)?;

            self.compile_terminator(
                &mut fn_ctx,
                &mlir_blocks,
                mlir_block,
                &block.terminator,
                helper,
            )?;
        }

        // Generate the function itself and add it to the module.
        self.module.body().append_operation(func::func(
            self.ctx,
            StringAttribute::new(self.ctx, &format!("mathic__{}", ir_func.name)),
            TypeAttribute::new(FunctionType::new(self.ctx, &params_types, &[return_ty]).into()),
            region,
            attributes,
            location,
        ));

        Ok(())
    }

    pub fn compile_block<'ctx, 'func>(
        &'func self,
        fn_ctx: &mut FunctionCtx<'func>,
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
