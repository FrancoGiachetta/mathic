use melior::{
    dialect::func,
    helpers::{BuiltinBlockExt, LlvmBlockExt},
    ir::{
        Attribute, Block, BlockLike, BlockRef, Identifier, Region, RegionLike, Type, TypeLike,
        Value, ValueLike,
        attribute::{StringAttribute, TypeAttribute},
        r#type::FunctionType,
    },
};
use mlir_sys::{MlirType, MlirValue};

use crate::{
    codegen::{MathicCodeGen, compiler_helper::CompilerHelper},
    diagnostics::CodegenError,
    lowering::ir::function::{Function, LocalKind},
};

/// Helper struct to store the current context of the function being compiled.
///
/// ## Fields
///
/// **locals**: variables defined within the function context.
/// **mlir_blocks**: the MLIR Blocks that the function will use.
pub struct FunctionCtx<'ctx, 'this> {
    locals: Vec<(MlirValue, MlirType)>,
    mlir_blocks: &'this [BlockRef<'ctx, 'this>],
}

impl<'ctx, 'this> FunctionCtx<'ctx, 'this> {
    pub fn new(mlir_blocks: &'this [BlockRef<'ctx, 'this>]) -> Self {
        Self {
            locals: Vec::new(),
            mlir_blocks,
        }
    }

    pub fn define_local(&mut self, value: Value, ty: Type) {
        self.locals.push((value.to_raw(), ty.to_raw()));
    }

    pub fn get_local(&self, idx: usize) -> Option<(Value<'ctx, '_>, Type<'ctx>)> {
        self.locals
            .get(idx)
            .copied()
            .map(|(v, t)| unsafe { (Value::from_raw(v), Type::from_raw(t)) })
    }

    pub fn get_block(&self, idx: usize) -> BlockRef<'_, '_> {
        *self.mlir_blocks.get(idx).expect("invalid block index")
    }
}

impl MathicCodeGen<'_> {
    pub fn compile_function<'ctx, 'func>(
        &'func self,
        inner_func: &Function,
        attributes: &[(Identifier<'_>, Attribute<'_>)],
        helper: &mut CompilerHelper,
    ) -> Result<(), CodegenError>
    where
        'func: 'ctx,
    {
        let location = self.get_location(None)?;

        let return_ty = inner_func.return_ty.get_compiled_type(self.ctx);
        let mut params_types = Vec::with_capacity(inner_func.params_tys.len());
        let mut block_params = Vec::with_capacity(inner_func.params_tys.len());

        // Prepare the function's params' types and the entry block params as
        // well.
        for param_ty in inner_func.params_tys.iter() {
            let mlir_ty = param_ty.get_compiled_type(self.ctx);

            params_types.push(mlir_ty);
            block_params.push((mlir_ty, location));
        }

        let region = Region::new();

        let mut mlir_blocks = Vec::with_capacity(inner_func.basic_blocks.len() - 1);

        // Create the entry block, the first block to be executed of every
        // function.
        let entry_block = {
            let block = region.append_block(Block::new(&block_params));

            mlir_blocks.push(block);

            block
        };

        // We already know the amount of blocks this function will use from the
        // lowering phase.
        for _ in 0..inner_func.basic_blocks.len() - 1 {
            mlir_blocks.push(region.append_block(Block::new(&[])));
        }

        let mut inner_fn_ctx = FunctionCtx::new(&mlir_blocks);
        let function_params = inner_func
            .sym_table
            .locals
            .iter()
            .filter(|l| l.kind == LocalKind::Param);

        {
            // Allocate space for params and make them visible to the function.
            for (i, _) in function_params.enumerate() {
                let value = entry_block.arg(i)?;
                let ptr = entry_block.alloca1(self.ctx, location, params_types[i], 8)?;

                entry_block.store(self.ctx, location, ptr, value)?;

                inner_fn_ctx
                    .define_local(ptr, inner_func.params_tys[i].get_compiled_type(self.ctx));
            }
        }

        // Precompile inner functions.
        for (_, inner_func) in inner_func.sym_table.functions.iter() {
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
        for (block, mlir_block) in inner_func.basic_blocks.iter().zip(&mlir_blocks) {
            self.compile_block(&mut inner_fn_ctx, mlir_block, &block.instructions, helper)?;

            self.compile_terminator(&mut inner_fn_ctx, mlir_block, &block.terminator, helper)?;
        }

        // Generate the function itself and add it to the module.
        self.module.body().append_operation(func::func(
            self.ctx,
            StringAttribute::new(self.ctx, &format!("mathic__{}", inner_func.name)),
            TypeAttribute::new(FunctionType::new(self.ctx, &params_types, &[return_ty]).into()),
            region,
            attributes,
            location,
        ));

        Ok(())
    }
}
