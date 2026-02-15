use melior::{Context, ir::Module};
use mlir_sys::MlirModule;

use crate::{
    MathicResult, codegen::error::CodegenError, error::MathicError, ffi, parser::ast::Program,
};

pub mod control_flow;
pub mod declaration;
pub mod error;
pub mod expression;
pub mod statement;

pub struct MathicCodeGen {
    ctx: Context,
    module: MlirModule,
}

impl MathicCodeGen {
    pub fn new() -> Result<Self, CodegenError> {
        let ctx = ffi::create_context()?;
        let module = ffi::create_module(&ctx)?;

        Ok(Self { ctx, module })
    }

    pub fn module(&self) -> Module<'_> {
        unsafe { Module::from_raw(self.module) }
    }

    pub fn ctx(&self) -> &Context {
        &self.ctx
    }

    pub fn generate_module(&self, program: Program) -> MathicResult<MlirModule> {
        // Check if main function is present
        if !program.funcs.iter().any(|f| f.name == "main") {
            return Err(MathicError::Codegen(CodegenError::MissingMainFunction));
        }

        // TODO: Compile structs in the future

        for func in program.funcs {
            self.compile_function(func)?;
            dbg!("DONE");
        }

        Ok(self.module)
    }
}
