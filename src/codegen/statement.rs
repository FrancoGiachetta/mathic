use llvm_sys::target::LLVMInitializeHexagonTarget;
use melior::{
    dialect::llvm,
    ir::{
        Block, BlockLike, Region, RegionLike,
        attribute::{StringAttribute, TypeAttribute},
        r#type::FunctionType,
    },
};

use crate::{
    codegen::{MathicCodeGen, error::CodegenError},
    error::{MathicError, Result},
    parser::grammar::{
        declaration::FuncDecl,
        statement::{ReturnStmt, Stmt},
    },
};

impl MathicCodeGen {
    fn compile_statement<'ctx: 'this, 'this>(
        &self,
        block: &'this Block<'ctx>,
        stmt: Stmt,
    ) -> Result<()> {
        match stmt {
            Stmt::Decl(decl_stmt) => Err(MathicError::Codegen(CodegenError::MeliorError(
                melior::Error::Operation("Declaration not implemented".into()),
            ))),
            Stmt::Block(block_stmt) => Err(MathicError::Codegen(CodegenError::MeliorError(
                melior::Error::Operation("Block statement not implemented".into()),
            ))),
            Stmt::Return(return_stmt) => self.compile_return(block, return_stmt),
            Stmt::Expr(expr_stmt) => Err(MathicError::Codegen(CodegenError::MeliorError(
                melior::Error::Operation("Expression statement not implemented".into()),
            ))),
        }
    }

    pub fn compile_function(&self, func: &FuncDecl) -> Result<()> {
        let params = vec![];

        let region = Region::new();
        let block = region.append_block(Block::new(&[]));

        for stmt in func.body {
            self.compile_statement(block, stmt)?;
        }

        let location = Location::unknown(&self.context);
        let i64_type = melior::ir::r#type::IntegerType::new(&self.context, 64).into();
        let func_ty = TypeAttribute::new(FunctionType::new(&self.context, &[], &[i64_type]));

        self.module.body().append_operation(llvm::func(
            &self.context,
            StringAttribute::new(&self.context, &func.name),
            func_ty,
            region,
            &[],
            location,
        ));
    }

    fn compile_return(&self, block: &'this Block<'ctx>, return_stmt: ReturnStmt) -> Result<()> {
        let value = self.compile_expression(block, return_stmt.value)?;
        let location = Location::unknown(&self.context);

        block.append_operation(llvm::r#return(&[&value], &location));
        Ok(())
    }
}
