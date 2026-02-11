use melior::{
    Context,
    dialect::func,
    ir::{
        Attribute, Block, BlockLike, Identifier, Location, Region, RegionLike,
        attribute::{StringAttribute, TypeAttribute},
        r#type::{FunctionType, IntegerType},
    },
};

use crate::{
    codegen::{MathicCodeGen, error::CodegenError},
    parser::grammar::{
        declaration::FuncDecl,
        statement::{ReturnStmt, Stmt},
    },
};

impl<'this, 'ctx> MathicCodeGen<'this, 'ctx>
where
    'this: 'ctx,
{
    fn compile_statement(
        &self,
        ctx: &'ctx Context,
        block: &'this Block<'ctx>,
        stmt: Stmt,
    ) -> Result<(), CodegenError> {
        match stmt {
            Stmt::Decl(_decl_stmt) => unimplemented!("Declaration not implemented"),
            Stmt::Block(_block_stmt) => unimplemented!("Block statement not implemented"),
            Stmt::Return(return_stmt) => self.compile_return(ctx, block, return_stmt),
            Stmt::Expr(_expr_stmt) => unimplemented!("Expression statement not implemented"),
        }
    }

    pub fn compile_function(&self, ctx: &'ctx Context, func: FuncDecl) -> Result<(), CodegenError> {
        // let params = vec![];

        let region = Region::new();
        let block = region.append_block(Block::new(&[]));

        for stmt in func.body {
            self.compile_statement(ctx, &block, stmt)?;
        }

        let location = Location::unknown(ctx);
        let i64_type = IntegerType::new(ctx, 64).into();

        self.module.body().append_operation(func::func(
            ctx,
            StringAttribute::new(ctx, &format!("mathic_{}", func.name)),
            TypeAttribute::new(FunctionType::new(ctx, &[], &[i64_type]).into()),
            region,
            // This is necessary for the ExecutorEngine to execute a function.
            &[(
                Identifier::new(ctx, "llvm.emit_c_interface"),
                Attribute::unit(ctx),
            )],
            location,
        ));

        Ok(())
    }

    fn compile_return(
        &self,
        ctx: &'ctx Context,
        block: &'this Block<'ctx>,
        return_stmt: ReturnStmt,
    ) -> Result<(), CodegenError> {
        let value = self.compile_expression(ctx, block, return_stmt.value)?;
        let location = Location::unknown(ctx);

        block.append_operation(func::r#return(&[value], location));
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::compile_and_execute;
    use rstest::*;

    #[rstest]
    #[case("df main() { return 0; }", 0)]
    #[case("df main() { return 42; }", 42)]
    #[case("df main() { return true; }", 1)]
    #[case("df main() { return false; }", 0)]
    fn test_return_statements(#[case] source: &str, #[case] expected: i64) {
        assert_eq!(compile_and_execute(source), expected);
    }

    #[rstest]
    #[case("df main() { return 42 == 42; }", 1)]
    #[case("df main() { return 42 != 21; }", 1)]
    #[case("df main() { return true and false; }", 0)]
    #[case("df main() { return true or false; }", 1)]
    fn test_return_with_expressions(#[case] source: &str, #[case] expected: i64) {
        assert_eq!(compile_and_execute(source), expected);
    }
}
