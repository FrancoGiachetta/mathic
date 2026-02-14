use melior::{
    Context,
    dialect::func,
    ir::{Block, BlockLike, Location},
};

use crate::{
    codegen::{MathicCodeGen, error::CodegenError},
    parser::ast::{expression::ExprStmt, statement::Stmt},
};

impl<'ctx> MathicCodeGen<'_, 'ctx> {
    pub fn compile_statement<'func>(
        &self,
        ctx: &'ctx Context,
        block: &'func Block<'ctx>,
        stmt: &Stmt,
    ) -> Result<(), CodegenError> {
        match stmt {
            Stmt::Decl(decl_stmt) => self.compile_declaration(ctx, block, decl_stmt),
            Stmt::Block(_block_stmt) => unimplemented!("Block statement not implemented"),
            Stmt::If(if_stmt) => self.compile_if(ctx, block, if_stmt),
            Stmt::While(while_stmt) => self.compile_while(ctx, block, while_stmt),
            Stmt::For(for_stmt) => self.compile_for(ctx, block, for_stmt),
            Stmt::Return(expr) => self.compile_return(ctx, block, expr),
            Stmt::Expr(_expr_stmt) => unimplemented!("Expression statement not implemented"),
        }
    }

    pub fn compile_block<'func>(
        &self,
        ctx: &'ctx Context,
        block: &'func Block<'ctx>,
        stmts: &[Stmt],
    ) -> Result<(), CodegenError> {
        for stmt in stmts {
            self.compile_statement(ctx, block, stmt)?;
        }

        Ok(())
    }

    fn compile_return<'func>(
        &self,
        ctx: &'ctx Context,
        block: &'func Block<'ctx>,
        expr: &ExprStmt,
    ) -> Result<(), CodegenError> {
        let value = self.compile_expression(ctx, block, expr)?;
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
