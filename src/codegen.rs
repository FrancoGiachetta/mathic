use melior::{Context, ir::Module};

use crate::{
    MathicResult, codegen::error::CodegenError, error::MathicError, parser::grammar::Program,
};

pub mod error;
pub mod expression;
pub mod statement;

pub struct MathicCodeGen<'this, 'ctx>
where
    'this: 'ctx,
{
    module: &'this Module<'ctx>,
}

impl<'this, 'ctx> MathicCodeGen<'this, 'ctx>
where
    'this: 'ctx,
{
    pub fn new(module: &'this Module<'ctx>) -> Self {
        Self { module }
    }

    pub fn generate_module(&mut self, ctx: &'ctx Context, program: Program) -> MathicResult<()> {
        // Check if main function is present
        if !program.funcs.iter().any(|f| f.name == "main") {
            return Err(MathicError::Codegen(CodegenError::MissingMainFunction));
        }

        // TODO: Compile structs in the future

        for func in program.funcs {
            self.compile_function(ctx, func)?;
        }

        Ok(())
    }
}
