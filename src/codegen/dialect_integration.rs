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

    use melior::{
        Context,
        ir::{Type, TypeLike},
    };

    use crate::ffi::dialect_integration::symbolic_dialect;

    pub mod operation {
        use melior::{
            Context,
            ir::{
                Identifier, Location, Operation, Type, Value, ValueLike,
                attribute::StringAttribute, operation::OperationBuilder,
            },
        };

        pub fn sym<'ctx>(
            ctx: &'ctx Context,
            location: Location<'ctx>,
            name: &str,
            result_type: Type<'ctx>,
        ) -> Operation<'ctx> {
            OperationBuilder::new("symbolic.sym", location)
                .add_attributes(&[(
                    Identifier::new(ctx, "name"),
                    StringAttribute::new(ctx, name).into(),
                )])
                .add_results(&[result_type])
                .build()
                .expect("valid operation")
        }

        pub fn add<'ctx>(
            location: Location<'ctx>,
            lhs: Value<'ctx, '_>,
            rhs: Value<'ctx, '_>,
            result_type: Type<'ctx>,
        ) -> Operation<'ctx> {
            OperationBuilder::new("symbolic.add", location)
                .add_operands(&[lhs, rhs])
                .add_results(&[result_type])
                .build()
                .expect("valid operation")
        }

        pub fn sub<'ctx>(
            location: Location<'ctx>,
            lhs: Value<'ctx, '_>,
            rhs: Value<'ctx, '_>,
            result_type: Type<'ctx>,
        ) -> Operation<'ctx> {
            OperationBuilder::new("symbolic.sub", location)
                .add_operands(&[lhs, rhs])
                .add_results(&[result_type])
                .build()
                .expect("valid operation")
        }

        pub fn mul<'ctx>(
            location: Location<'ctx>,
            lhs: Value<'ctx, '_>,
            rhs: Value<'ctx, '_>,
            result_type: Type<'ctx>,
        ) -> Operation<'ctx> {
            OperationBuilder::new("symbolic.mul", location)
                .add_operands(&[lhs, rhs])
                .add_results(&[result_type])
                .build()
                .expect("valid operation")
        }

        pub fn div<'ctx>(
            location: Location<'ctx>,
            lhs: Value<'ctx, '_>,
            rhs: Value<'ctx, '_>,
            result_type: Type<'ctx>,
        ) -> Operation<'ctx> {
            OperationBuilder::new("symbolic.div", location)
                .add_operands(&[lhs, rhs])
                .add_results(&[result_type])
                .build()
                .expect("valid operation")
        }

        pub fn eval<'ctx>(
            ctx: &'ctx Context,
            location: Location<'ctx>,
            expr: Value<'ctx, '_>,
            sym_name: &str,
            value: Value<'ctx, '_>,
        ) -> Operation<'ctx> {
            let result_type = value.r#type();
            OperationBuilder::new("symbolic.eval", location)
                .add_operands(&[expr, value])
                .add_attributes(&[(
                    Identifier::new(ctx, "sym"),
                    StringAttribute::new(ctx, sym_name).into(),
                )])
                .add_results(&[result_type])
                .build()
                .expect("valid operation")
        }
    }

    pub fn sym_expr_type<'ctx>(
        ctx: &'ctx Context,
        inner_type: Type<'ctx>,
        is_signed: bool,
    ) -> Type<'ctx> {
        unsafe {
            Type::from_raw(symbolic_dialect::getSymExprType(
                ctx.to_raw(),
                inner_type.to_raw(),
                is_signed,
            ))
        }
    }
}
