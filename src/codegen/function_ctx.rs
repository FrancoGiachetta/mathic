use melior::ir::{BlockRef, Value, ValueLike};
use mlir_sys::MlirValue;

pub struct FunctionCtx<'ctx, 'this> {
    locals: Vec<MlirValue>,
    mlir_blocks: &'this [BlockRef<'ctx, 'this>],
}

impl<'ctx, 'this> FunctionCtx<'ctx, 'this> {
    pub fn new(mlir_blocks: &'this [BlockRef<'ctx, 'this>]) -> Self {
        Self {
            locals: Vec::new(),
            mlir_blocks,
        }
    }

    pub fn define_local(&mut self, value: Value) {
        self.locals.push(value.to_raw());
    }

    pub fn get_local(&self, idx: usize) -> Option<Value<'ctx, '_>> {
        self.locals
            .get(idx)
            .copied()
            .map(|v| unsafe { Value::from_raw(v) })
    }

    pub fn get_block(&self, idx: usize) -> BlockRef<'_, '_> {
        *self.mlir_blocks.get(idx).expect("invalid block index")
    }
}
