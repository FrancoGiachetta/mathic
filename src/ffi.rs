use std::sync::OnceLock;

use melior::{
    Context,
    dialect::DialectRegistry,
    ir::{
        Block, Identifier, Location, Module, Region, RegionLike, attribute::StringAttribute,
        operation::OperationBuilder,
    },
    utility::{register_all_dialects, register_all_llvm_translations, register_all_passes},
};

use crate::{
    compiler::OptLvl,
    diagnostics::CodegenError,
    ffi::llvm::{get_data_layout_rep, get_target_triple, initialize_llvm},
};

pub mod dialect_integration;
pub mod llvm;

pub fn create_module<'ctx>(
    ctx: &'ctx Context,
    opt_lvl: OptLvl,
) -> Result<Module<'ctx>, CodegenError> {
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

    Module::from_operation(op).ok_or(CodegenError::Custom("Could not create module".to_string()))
}

pub fn create_context() -> Result<Context, CodegenError> {
    let ctx = Context::new();

    ctx.append_dialect_registry(&create_dialect_registry());
    ctx.load_all_available_dialects();

    register_all_passes();
    register_all_llvm_translations(&ctx);

    Ok(ctx)
}

fn create_dialect_registry() -> DialectRegistry {
    let registry = DialectRegistry::new();

    dialect_integration::symbolic_dialect::register_symbolic_dialect(&registry);
    register_all_dialects(&registry);

    registry
}
