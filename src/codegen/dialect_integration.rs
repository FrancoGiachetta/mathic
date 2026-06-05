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
        use melior::{
            Context,
            ir::{Location, Operation, operation::OperationBuilder},
        };

        pub fn add<'ctx>(ctx: &'ctx Context, location: Location<'ctx>) -> Operation<'ctx> {
            OperationBuilder::new("symbolic.add", location)
                .build()
                .expect("valid operation")
        }

        pub fn sub<'ctx>(ctx: &'ctx Context, location: Location<'ctx>) -> Operation<'ctx> {
            OperationBuilder::new("symbolic.sub", location)
                .build()
                .expect("valid operation")
        }

        pub fn mul<'ctx>(ctx: &'ctx Context, location: Location<'ctx>) -> Operation<'ctx> {
            OperationBuilder::new("symbolic.mul", location)
                .build()
                .expect("valid operation")
        }

        pub fn div<'ctx>(ctx: &'ctx Context, location: Location<'ctx>) -> Operation<'ctx> {
            OperationBuilder::new("symbolic.div", location)
                .build()
                .expect("valid operation")
        }

        pub fn eval<'ctx>(ctx: &'ctx Context, location: Location<'ctx>) -> Operation<'ctx> {
            OperationBuilder::new("symbolic.eval", location)
                .build()
                .expect("valid operation")
        }
    }

    pub fn sym_expr_type(registry: DialectRegistry) {
        unsafe {
            symbolic_dialect::getSymExprType(registry.to_raw());
        }
    }
}
