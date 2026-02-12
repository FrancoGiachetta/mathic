use melior::{Context, ir::Block};

use crate::{
    codegen::{MathicCodeGen, error::CodegenError},
    parser::grammar::control_flow::{ForStmt, IfStmt, WhileStmt},
};

impl<'this, 'ctx> MathicCodeGen<'this, 'ctx>
where
    'this: 'ctx,
{
    pub fn compile_if(
        &self,
        ctx: &'ctx Context,
        block: &'this Block<'ctx>,
        stmt: IfStmt,
    ) -> Result<(), CodegenError> {
        unimplemented!("If statement codegen not implemented")
    }

    pub fn compile_while(
        &self,
        ctx: &'ctx Context,
        block: &'this Block<'ctx>,
        stmt: WhileStmt,
    ) -> Result<(), CodegenError> {
        unimplemented!("While loop codegen not implemented")
    }

    pub fn compile_for(
        &self,
        ctx: &'ctx Context,
        block: &'this Block<'ctx>,
        stmt: ForStmt,
    ) -> Result<(), CodegenError> {
        unimplemented!("For loop codegen not implemented")
    }
}
