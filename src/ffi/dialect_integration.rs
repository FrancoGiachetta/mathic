pub mod symbolic_dialect {
    use melior::{dialect::DialectRegistry, pass::Pass};
    use mlir_sys::{MlirContext, MlirDialectRegistry, MlirPass, MlirType};

    #[link(name = "dialect_bindings")]
    unsafe extern "C" {
        fn mlirCreateSymbolicExtractEval() -> MlirPass;
        fn mlirCreateSymbolicToArith() -> MlirPass;
        fn mlirInsertSymbolicDialect(registry: MlirDialectRegistry);
        pub fn getSymExprType(ctx: MlirContext, inner_type: MlirType, is_signed: bool) -> MlirType;
    }

    pub fn register_symbolic_dialect(registry: &DialectRegistry) {
        unsafe {
            mlirInsertSymbolicDialect(registry.to_raw());
        }
    }
    pub fn create_symbolic_to_arith() -> Pass {
        unsafe { Pass::from_raw_fn(mlirCreateSymbolicToArith) }
    }
    pub fn create_symbolic_extract_eval() -> Pass {
        unsafe { Pass::from_raw_fn(mlirCreateSymbolicExtractEval) }
    }
}
