use melior::{
    dialect::func,
    helpers::{BuiltinBlockExt, LlvmBlockExt},
    ir::{Block, BlockLike, Location, ValueLike},
};

use crate::{
    codegen::{error::CodegenError, MathicCodeGen},
    parser::ast::{expression::ExprStmt, statement::Stmt},
};

impl MathicCodeGen<'_> {
    pub fn compile_statement(&self, block: &Block, stmt: &Stmt) -> Result<(), CodegenError> {
        match stmt {
            Stmt::Decl(decl_stmt) => self.compile_declaration(block, decl_stmt),
            Stmt::Block(_block_stmt) => unimplemented!("Block statement not implemented"),
            Stmt::If(if_stmt) => self.compile_if(block, if_stmt),
            Stmt::While(while_stmt) => self.compile_while(block, while_stmt),
            Stmt::For(for_stmt) => self.compile_for(block, for_stmt),
            Stmt::Return(return_stmt) => self.compile_return(block, return_stmt),
            Stmt::Assign { name, value } => self.compile_assignment(block, name, value),
        }
    }

    pub fn compile_block(&self, block: &Block, stmts: &[Stmt]) -> Result<(), CodegenError> {
        for stmt in stmts {
            self.compile_statement(block, stmt)?;
        }

        Ok(())
    }

    fn compile_return(&self, block: &Block, expr: &ExprStmt) -> Result<(), CodegenError> {
        let value = self.compile_expression(block, expr)?;
        let location = Location::unknown(self.ctx);

        block.append_operation(func::r#return(&[value], location));

        Ok(())
    }

    fn compile_assignment(
        &self,
        block: &Block,
        name: &str,
        expr: &ExprStmt,
    ) -> Result<(), CodegenError> {
        let location = Location::unknown(self.ctx);
        let value = self.compile_expression(block, expr)?;

        let ptr = self.get_sym(name)?;

        block.store(self.ctx, location, ptr, value)?;

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
