use std::sync::OnceLock;

use melior::{
    Context,
    ir::{
        Block, Identifier, Location, Module, OperationRef, Region, RegionLike,
        attribute::StringAttribute, operation::OperationBuilder,
    },
};

use crate::{
    compiler::OptLvl,
    diagnostics::CodegenError,
    ffi::llvm::{get_data_layout_rep, get_target_triple, initialize_llvm},
};

pub struct MathicModule<'ctx> {
    inner: Module<'ctx>,
}

unsafe impl Sync for MathicModule<'_> {}
unsafe impl Send for MathicModule<'_> {}

impl<'ctx> MathicModule<'ctx> {
    pub fn new(ctx: &'ctx Context, opt_lvl: OptLvl) -> Result<Self, CodegenError> {
        static INITIALIZED: OnceLock<()> = OnceLock::new();

        INITIALIZED.get_or_init(initialize_llvm);

        let target_triple = get_target_triple();

        let module_region = Region::new();
        module_region.append_block(Block::new(&[]));

        let data_layout_ret = &get_data_layout_rep(opt_lvl.into())?;

        let op = OperationBuilder::new("builtin.module", Location::unknown(ctx))
            .add_attributes(&[
                (
                    Identifier::new(ctx, "llvm.target_triple"),
                    StringAttribute::new(ctx, &target_triple).into(),
                ),
                (
                    Identifier::new(ctx, "llvm.data_layout"),
                    StringAttribute::new(ctx, data_layout_ret).into(),
                ),
            ])
            .add_regions([module_region])
            .build()?;

        Ok(Self {
            inner: Module::from_operation(op)
                .ok_or(CodegenError::Custom("Could not create module".to_string()))?,
        })
    }

    pub fn as_inner(&self) -> &Module<'_> {
        &self.inner
    }

    pub fn as_inner_mut(&mut self) -> &mut Module<'ctx> {
        &mut self.inner
    }

    pub fn inner_to_operation(&'_ self) -> OperationRef<'ctx, '_> {
        self.inner.as_operation()
    }

    pub fn inner_owned(self) -> Module<'ctx> {
        self.inner
    }
}
