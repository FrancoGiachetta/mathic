use melior::{
    helpers::{BuiltinBlockExt, LlvmBlockExt},
    ir::{Block, BlockRef, Location, Region, RegionLike, Type, Value, ValueLike},
};
use mlir_sys::MlirValue;

use crate::{
    codegen::MathicCodeGen,
    diagnostics::CodegenError,
    lowering::ir::{
        basic_block::Terminator,
        function::{Function, LocalKind},
        symbols::TypeIndex,
    },
};

/// Helper struct to store the current context of the function being compiled.
///
/// ## Fields
///
/// **locals**: variables defined within the function context. Stores either
/// symbols or pointers to stack allocated variables.
/// **mlir_blocks**: the MLIR Blocks that the function will use.
#[derive(Debug)]
pub struct FunctionCtx<'this> {
    locals: Vec<(MlirValue, TypeIndex)>,
    ir_func: &'this Function,
}

impl<'ctx, 'this> FunctionCtx<'this> {
    pub fn define_local(&mut self, value: Value, ty: TypeIndex) {
        self.locals.push((value.to_raw(), ty));
    }

    pub fn assign_local(&mut self, idx: usize, value: Value<'ctx, '_>) {
        if let Some(entry) = self.locals.get_mut(idx) {
            *entry = (value.to_raw(), entry.1);
        }
    }

    pub fn get_local(&self, idx: usize) -> Result<(Value<'ctx, 'this>, TypeIndex), CodegenError> {
        self.locals
            .get(idx)
            .copied()
            .map(|(v, t)| (unsafe { Value::from_raw(v) }, t))
            .ok_or(CodegenError::Custom(format!(
                "Could not find local with idx: {}",
                idx
            )))
    }

    pub fn get_ir_func(&self) -> &Function {
        self.ir_func
    }
}

impl MathicCodeGen<'_> {
    pub fn create_fn_ctx<'ctx, 'func>(
        &'func self,
        region: &Region<'ctx>,
        location: Location<'ctx>,
        ir_func: &'func Function,
        entry_block_params: &[(Type<'ctx>, Location<'ctx>)],
    ) -> Result<(FunctionCtx<'func>, Vec<BlockRef<'ctx, 'func>>), CodegenError>
    where
        'func: 'ctx,
    {
        let mut mlir_blocks = Vec::with_capacity(ir_func.basic_blocks.len() - 1);

        // Create the entry block, the first block to be executed of every
        // function.
        let entry_block = {
            let block = region.append_block(Block::new(entry_block_params));

            mlir_blocks.push(block);

            block
        };

        // We already know the amount of blocks this function will use from the
        // lowering phase.
        let mut i = 0;
        while i < ir_func.basic_blocks.len() - 1 {
            match &ir_func.basic_blocks[i].terminator {
                Terminator::CondBranch {
                    true_block,
                    false_block,
                    true_block_args,
                    false_block_args,
                    ..
                } => {
                    let true_block_args =
                        self.compile_locals_types(true_block_args, ir_func, location)?;
                    let false_block_args =
                        self.compile_locals_types(false_block_args, ir_func, location)?;

                    mlir_blocks.insert(
                        *true_block,
                        region.append_block(Block::new(&true_block_args)),
                    );
                    mlir_blocks.insert(
                        *false_block,
                        region.append_block(Block::new(&false_block_args)),
                    );

                    // Already created the true succesor block.
                    i += 2;
                }
                Terminator::Branch {
                    target, block_args, ..
                } => {
                    let block_args = self.compile_locals_types(block_args, ir_func, location)?;

                    mlir_blocks.insert(*target, region.append_block(Block::new(&block_args)));

                    i += 1;

                    continue;
                }
                _ => {
                    mlir_blocks.push(region.append_block(Block::new(&[])));

                    i += 1;
                }
            }
        }

        let mut fn_ctx = FunctionCtx {
            locals: Vec::new(),
            ir_func,
        };
        let function_params = ir_func
            .get_locals()
            .iter()
            .filter(|l| l.kind == LocalKind::Param);

        {
            // Allocate space for params and make them visible to the function.
            for (i, _) in function_params.enumerate() {
                let value = entry_block.arg(i)?;
                let ptr = entry_block.alloca1(self.ctx, location, entry_block_params[i].0, 8)?;

                entry_block.store(self.ctx, location, ptr, value)?;

                fn_ctx.define_local(ptr, ir_func.params_tys[i]);
            }
        }

        Ok((fn_ctx, mlir_blocks))
    }

    fn compile_locals_types<'ctx, 'func>(
        &'func self,
        local_indexes: &[usize],
        ir_func: &'func Function,
        location: Location<'ctx>,
    ) -> Result<Vec<(Type<'ctx>, Location<'ctx>)>, CodegenError>
    where
        'func: 'ctx,
    {
        Ok(local_indexes
            .iter()
            .map(|local_idx| {
                let local = ir_func.get_local(*local_idx).expect("invalid local idx");

                self.get_compiled_type(ir_func, local.ty)
            })
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .map(|ty| (ty, location))
            .collect::<Vec<_>>())
    }
}
