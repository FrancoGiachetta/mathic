use melior::{
    dialect::llvm,
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
    fn compile_statement(&self, block: &'this Block<'ctx>, stmt: Stmt) -> Result<(), CodegenError> {
        match stmt {
            Stmt::Decl(_decl_stmt) => unimplemented!("Declaration not implemented"),
            Stmt::Block(_block_stmt) => unimplemented!("Block statement not implemented"),
            Stmt::Return(return_stmt) => self.compile_return(block, return_stmt),
            Stmt::Expr(_expr_stmt) => unimplemented!("Expression statement not implemented"),
        }
    }

    pub fn compile_function(&self, func: FuncDecl) -> Result<(), CodegenError> {
        // let params = vec![];

        let region = Region::new();
        let block = region.append_block(Block::new(&[]));

        for stmt in func.body {
            self.compile_statement(&block, stmt)?;
        }

        let location = Location::unknown(&self.ctx);
        let i64_type = IntegerType::new(&self.ctx, 64).into();
        let func_ty = TypeAttribute::new(llvm::r#type::function(i64_type, &[], false).into());

        self.module.body().append_operation(llvm::func(
            &self.ctx,
            StringAttribute::new(&self.ctx, &func.name),
            func_ty,
            region,
            &[
                (
                    Identifier::new(&self.ctx, "sym_visibility"),
                    StringAttribute::new(&self.ctx, "private").into(),
                ),
                (
                    Identifier::new(&self.ctx, "linkage"),
                    Attribute::parse(&self.ctx, "#llvm.linkage<private>")
                        .ok_or(CodegenError::ParseAttributeError)?,
                ),
                (
                    Identifier::new(&self.ctx, "CConv"),
                    Attribute::parse(&self.ctx, "#llvm.cconv<fastcc>")
                        .ok_or(CodegenError::ParseAttributeError)?,
                ),
            ],
            location,
        ));

        Ok(())
    }

    fn compile_return(
        &self,
        block: &'this Block<'ctx>,
        return_stmt: ReturnStmt,
    ) -> Result<(), CodegenError> {
        let value = self.compile_expression(block, return_stmt.value)?;
        let location = Location::unknown(&self.ctx);

        block.append_operation(llvm::r#return(Some(value), location));
        Ok(())
    }
}
