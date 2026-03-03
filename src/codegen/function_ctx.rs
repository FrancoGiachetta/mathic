use melior::{
    dialect::func,
    helpers::{BuiltinBlockExt, LlvmBlockExt},
    ir::{
        Attribute, Block, BlockLike, BlockRef, Identifier, Region, RegionLike, Type, TypeLike,
        Value, ValueLike,
        attribute::{StringAttribute, TypeAttribute},
        r#type::{FunctionType, IntegerType},
    },
};
use mlir_sys::{MlirType, MlirValue};

use crate::{
    codegen::MathicCodeGen,
    diagnostics::CodegenError,
    lowering::ir::function::{Function, LocalKind},
};

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
    ) -> Result<(), CodegenError>
    where
        'func: 'ctx,
    {
        let location = self.get_location(None)?;
        let i64_ty = IntegerType::new(self.ctx, 64).into();

        let mut params_types = Vec::with_capacity(inner_func.params_types.len());
        let mut block_params = Vec::with_capacity(inner_func.params_types.len());

        for param_ty in inner_func.params_types.iter() {
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

        // Create the rest of the blocks.
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
            // Allocate space for params and make them visible to the function
            for (i, _) in function_params.enumerate() {
                let value = entry_block.arg(i)?;
                let ptr = entry_block.alloca1(self.ctx, location, params_types[i], 8)?;

                entry_block.store(self.ctx, location, ptr, value)?;

                inner_fn_ctx
                    .define_local(ptr, inner_func.params_types[i].get_compiled_type(self.ctx));
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
            )?;
        }

        // Generate code for every basic_block. For every block, we first
        // compile its instructions. After that, the block's terminator
        // instruction gets compiled.
        for (block, mlir_block) in inner_func.basic_blocks.iter().zip(&mlir_blocks) {
            self.compile_block(&mut inner_fn_ctx, mlir_block, &block.instructions)?;

            self.compile_terminator(&mut inner_fn_ctx, mlir_block, &block.terminator)?;
        }

        // Generate the function itself.
        self.module.body().append_operation(func::func(
            self.ctx,
            StringAttribute::new(self.ctx, &format!("mathic__{}", inner_func.name)),
            TypeAttribute::new(FunctionType::new(self.ctx, &params_types, &[i64_ty]).into()),
            region,
            attributes,
            location,
        ));

        Ok(())
    }
}
