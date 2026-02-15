use melior::{Context, ir::Module};

use crate::{MathicResult, codegen::error::CodegenError, error::MathicError, parser::ast::Program};

pub mod control_flow;
pub mod declaration;
pub mod error;
pub mod expression;
pub mod statement;

pub struct MathicCodeGen<'ctx> {
    pub ctx: &'ctx Context,
    pub module: &'ctx Module<'ctx>,
}

impl<'ctx> MathicCodeGen<'ctx> {
    pub fn generate_module(&self, program: Program) -> MathicResult<()> {
        // Check if main function is present
        if !program.funcs.iter().any(|f| f.name == "main") {
            return Err(MathicError::Codegen(CodegenError::MissingMainFunction));
        }

        // TODO: Compile structs in the future

        for func in program.funcs {
            self.compile_function(func)?;
            dbg!("DONE");
        }

        Ok(())
    }
}
