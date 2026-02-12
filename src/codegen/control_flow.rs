use melior::{Context, ir::Block};

use crate::codegen::{MathicCodeGen, error::CodegenError};

impl<'this, 'ctx> MathicCodeGen<'this, 'ctx>
where
    'this: 'ctx,
{
    pub fn compile_if(
        &self,
        _ctx: &'ctx Context,
        _block: &'this Block<'ctx>,
    ) -> Result<(), CodegenError> {
        unimplemented!("If statement codegen not implemented")
    }

    pub fn compile_while(
        &self,
        _ctx: &'ctx Context,
        _block: &'this Block<'ctx>,
    ) -> Result<(), CodegenError> {
        unimplemented!("While loop codegen not implemented")
    }

    pub fn compile_for(
        &self,
        _ctx: &'ctx Context,
        _block: &'this Block<'ctx>,
    ) -> Result<(), CodegenError> {
        unimplemented!("For loop codegen not implemented")
    }
}
