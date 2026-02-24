use std::{fs, path::PathBuf};

use ariadne::Source;
use melior::{
    Context,
    dialect::{cf, func, llvm},
    helpers::{BuiltinBlockExt, LlvmBlockExt},
    ir::{
        Block, BlockLike, Identifier, Location, Module, Region, RegionLike,
        attribute::{Attribute, FlatSymbolRefAttribute, StringAttribute, TypeAttribute},
        r#type::{FunctionType, IntegerType},
    },
};

use crate::{
    MathicResult,
    codegen::{error::CodegenError, function_ctx::FunctionCtx},
    error::MathicError,
    lowering::ir::{
        Ir,
        basic_block::Terminator,
        function::{Function, LocalKind},
    },
    parser::lexer::Span,
};

pub mod error;
pub mod function_ctx;
pub mod rvalue;
pub mod statement;

pub struct MathicCodeGen<'ctx> {
    ctx: &'ctx Context,
    module: &'ctx Module<'ctx>,
    file_path: Option<PathBuf>,
}

impl<'ctx> MathicCodeGen<'ctx> {
    pub fn new(ctx: &'ctx Context, module: &'ctx Module<'ctx>, file_path: Option<PathBuf>) -> Self {
        Self {
            ctx,
            module,
            file_path,
        }
    }

    pub fn get_location(&self, span: Option<Span>) -> Result<Location<'ctx>, CodegenError> {
        Ok(
            if let (Some(path), Some(span)) = (self.file_path.as_ref(), span) {
                let (_, line, column) = {
                    let source = fs::read_to_string(path)?;
                    Source::from(source).get_offset_line(span.start).unwrap()
                };
                Location::new(
                    self.ctx,
                    path.file_name().unwrap().to_str().unwrap(),
                    line,
                    column,
                )
            } else {
                Location::unknown(self.ctx)
            },
        )
    }

    pub fn generate_module(&self, program: &Ir) -> MathicResult<()> {
        // Check if main function is present
        if !program.functions.iter().any(|f| f.name == "main") {
            return Err(MathicError::Codegen(CodegenError::MissingMainFunction));
        }

        // TODO: Compile structs in the future

        for func in program.functions.iter() {
            self.compile_entry_point(func)?;
        }

        Ok(())
    }

    pub fn compile_entry_point(&self, func: &Function) -> Result<(), CodegenError> {
        let location = self.get_location(None)?;
        let i64_ty = IntegerType::new(self.ctx, 64).into();

        let function_params = func
            .sym_table
            .locals
            .iter()
            .filter(|l| l.kind == LocalKind::Param)
            .collect::<Vec<_>>();

        let mut params_types = Vec::with_capacity(function_params.len());
        let mut block_params = Vec::with_capacity(function_params.len());

        for _ in function_params.iter() {
            params_types.push(i64_ty);
            block_params.push((i64_ty, location));
        }

        let region = Region::new();

        let mut mlir_blocks = Vec::with_capacity(func.basic_blocks.len() - 1);

        // Create the entry block, the first block to be executed of every
        // function.
        let entry_block = {
            let block = region.append_block(Block::new(&block_params));

            mlir_blocks.push(block);

            block
        };

        // Create the rest of the blocks.
        for _ in 0..func.basic_blocks.len() - 1 {
            mlir_blocks.push(region.append_block(Block::new(&[])));
        }

        let mut fn_ctx = FunctionCtx::new(&mlir_blocks);

        {
            // Allocate space for params and make them visible to the function
            for (i, _) in func
                .sym_table
                .locals
                .iter()
                .filter(|l| l.kind == LocalKind::Param)
                .enumerate()
            {
                let value = entry_block.arg(i)?;
                let ptr = entry_block.alloca1(self.ctx, location, params_types[i], 8)?;

                entry_block.store(self.ctx, location, ptr, value)?;

                fn_ctx.define_local(ptr);
            }
        }

        // Generate code for every basic_block. For every block, we first
        // compile its instructions. After that, the block's terminator
        // instruction gets compiled.
        for (block, mlir_block) in func.basic_blocks.iter().zip(&mlir_blocks) {
            self.compile_block(&mut fn_ctx, mlir_block, &block.instructions)?;

            self.compile_terminator(&mut fn_ctx, mlir_block, &block.terminator)?;
        }

        // Generator the function itself.
        self.module.body().append_operation(func::func(
            self.ctx,
            StringAttribute::new(self.ctx, &format!("mathic__{}", func.name)),
            TypeAttribute::new(FunctionType::new(self.ctx, &params_types, &[i64_ty]).into()),
            region,
            // This is necessary for the ExecutorEngine to execute a function.
            &[(
                Identifier::new(self.ctx, "llvm.emit_c_interface"),
                Attribute::unit(self.ctx),
            )],
            location,
        ));

        Ok(())
    }

    fn compile_terminator<'func>(
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
}
