use std::{fs, path::PathBuf};

use ariadne::Source;
use melior::{
    Context,
    dialect::func,
    helpers::{BuiltinBlockExt, LlvmBlockExt},
    ir::{
        Block, BlockLike, Identifier, Location, Module, Region, RegionLike,
        attribute::{Attribute, StringAttribute, TypeAttribute},
        r#type::{FunctionType, IntegerType},
    },
};

use crate::{
    MathicResult,
    codegen::{error::CodegenError, function_ctx::FunctionCtx},
    error::MathicError,
    lowering::ir::{
        Ir,
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

        // Precompile inner functions .
        for (_, inner_func) in func.sym_table.functions.iter() {
            self.compile_inner_function(&mut fn_ctx, &entry_block, inner_func)?;
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
}
