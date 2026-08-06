use melior::{
    Context,
    dialect::DialectRegistry,
    utility::{register_all_dialects, register_all_llvm_translations, register_all_passes},
};

use crate::diagnostics::CodegenError;

pub mod dialect_integration;
pub mod llvm;

/// Creates an empty MLIR Module.
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
