use melior::{
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

        let location = Location::unknown(self.ctx);
        let i64_type = IntegerType::new(self.ctx, 64).into();

        self.module.body().append_operation(func::func(
            self.ctx,
            StringAttribute::new(self.ctx, &format!("mathic_{}", func.name)),
            TypeAttribute::new(FunctionType::new(self.ctx, &[], &[i64_type]).into()),
            region,
            // This is necessary for the ExecutorEngine to execute a function.
            &[(
                Identifier::new(self.ctx, "llvm.emit_c_interface"),
                Attribute::unit(self.ctx),
            )],
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
        let location = Location::unknown(self.ctx);

        block.append_operation(func::r#return(&[value], location));
        Ok(())
    }
}
