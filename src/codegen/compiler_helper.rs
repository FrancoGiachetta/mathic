#![allow(dead_code)]

use std::{any::TypeId, collections::HashMap};

use std::any::Any;

use melior::ir::attribute::DenseI32ArrayAttribute;
use melior::ir::{Identifier, Location};
use melior::{
    Context,
    ir::{Operation, Type, Value, operation::OperationBuilder},
};

use crate::diagnostics::CodegenError;

pub mod debugging;

pub struct CompilerHelper {
    map: HashMap<TypeId, Box<dyn Any>>,
}

impl CompilerHelper {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn get_or_insert<T: Any>(&mut self, init: impl FnOnce() -> T) -> &mut T {
        self.map
            .entry(TypeId::of::<T>())
            .or_insert_with(|| Box::new(init()))
            .downcast_mut::<T>()
            .expect("could not downcast")
    }
}

pub fn build_llvm_indirect_call<'ctx, 'func>(
    ctx: &'ctx Context,
    args: &[Value<'ctx, 'func>],
    results: &[Type<'ctx>],
) -> Result<Operation<'ctx>, CodegenError> {
    Ok(OperationBuilder::new("llvm.call", Location::unknown(ctx))
        .add_operands(args)
        .add_attributes(&[
            (
                Identifier::new(ctx, "operandSegmentSizes"),
                DenseI32ArrayAttribute::new(ctx, &[args.len() as i32, 0]).into(),
            ),
            (
                Identifier::new(ctx, "op_bundle_sizes"),
                DenseI32ArrayAttribute::new(ctx, &[]).into(),
            ),
        ])
        .add_results(results)
        .build()?)
}
