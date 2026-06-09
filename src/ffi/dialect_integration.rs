pub mod symbolic_dialect {
    use melior::dialect::DialectRegistry;
    use mlir_sys::{MlirContext, MlirDialectRegistry, MlirType};

    #[link(name = "dialect_bindings")]
    unsafe extern "C" {
        fn registerSymbolicDialect(registry: MlirDialectRegistry);
        pub fn getSymExprType(ctx: MlirContext) -> MlirType;
    }

    pub fn register_symbolic_dialect(registry: &DialectRegistry) {
        unsafe {
            registerSymbolicDialect(registry.to_raw());
        }
    }
}
