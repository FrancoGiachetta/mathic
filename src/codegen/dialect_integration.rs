pub mod symbolic {
    // melior::dialect! {
    //     name: "symbolic",
    //     files: [
    //         "IR/SymbolicDialect.td",
    //         "IR/SymbolicOps.td",
    //         "IR/SymbolicTypes.td",
    //         "Conversions/SymbolicToArith/SymbolicToArith.td",
    //         "Transforms/Passes.td",
    //     ],
    //     include_directories: [
    //         "./Dialects/include/Dialect/Symbolic/IR/",
    //         "./Dialects/include/Dialect/Symbolic/"
    //     ]
    // }

    use melior::dialect::DialectRegistry;

    use crate::ffi::dialect_integration::symbolic_dialect;

    pub mod operation {
        use melior::ir::{Location, Operation, operation::OperationBuilder};

        #[allow(dead_code)]
        pub fn add<'ctx>(location: Location<'ctx>) -> Operation<'ctx> {
            OperationBuilder::new("symbolic.add", location)
                .build()
                .expect("valid operation")
        }

        #[allow(dead_code)]
        pub fn sub<'ctx>(location: Location<'ctx>) -> Operation<'ctx> {
            OperationBuilder::new("symbolic.sub", location)
                .build()
                .expect("valid operation")
        }

        #[allow(dead_code)]
        pub fn mul<'ctx>(location: Location<'ctx>) -> Operation<'ctx> {
            OperationBuilder::new("symbolic.mul", location)
                .build()
                .expect("valid operation")
        }

        #[allow(dead_code)]
        pub fn div<'ctx>(location: Location<'ctx>) -> Operation<'ctx> {
            OperationBuilder::new("symbolic.div", location)
                .build()
                .expect("valid operation")
        }

        #[allow(dead_code)]
        pub fn eval<'ctx>(location: Location<'ctx>) -> Operation<'ctx> {
            OperationBuilder::new("symbolic.eval", location)
                .build()
                .expect("valid operation")
        }
    }

    #[allow(dead_code)]
    pub fn sym_expr_type(registry: DialectRegistry) {
        unsafe {
            symbolic_dialect::getSymExprType(registry.to_raw());
        }
    }
}
