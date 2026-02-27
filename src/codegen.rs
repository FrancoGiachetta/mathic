use std::{fs, path::PathBuf};

use ariadne::Source;
use melior::{
    Context,
    ir::{Identifier, Location, Module, attribute::Attribute},
};

use crate::{
    MathicResult, codegen::error::CodegenError, error::MathicError, lowering::ir::Ir,
    parser::lexer::Span,
};

pub mod error;
pub mod function_ctx;
pub mod rvalue;
pub mod statement;

pub struct MathicCodeGen<'ctx> {
    ctx: &'ctx Context,
    module: &'ctx Module<'ctx>,
    file_path: Option<PathBuf>,
}

impl<'ctx> MathicCodeGen<'ctx> {
    pub fn new(ctx: &'ctx Context, module: &'ctx Module<'ctx>, file_path: Option<PathBuf>) -> Self {
        Self {
            ctx,
            module,
            file_path,
        }
    }

    pub fn get_location(&self, span: Option<Span>) -> Result<Location<'ctx>, CodegenError> {
        Ok(
            if let (Some(path), Some(span)) = (self.file_path.as_ref(), span) {
                let (_, line, column) = {
                    let source = fs::read_to_string(path)?;
                    Source::from(source).get_offset_line(span.start).unwrap()
                };
                Location::new(
                    self.ctx,
                    path.file_name().unwrap().to_str().unwrap(),
                    line,
                    column,
                )
            } else {
                Location::unknown(self.ctx)
            },
        )
    }

    pub fn generate_module(&self, program: &Ir) -> MathicResult<()> {
        // Check if main function is present
        if !program.functions.iter().any(|f| f.name == "main") {
            return Err(MathicError::Codegen(CodegenError::MissingMainFunction));
        }

        // TODO: Compile structs in the future

        for func in program.functions.iter() {
            self.compile_function(
                func,
                &[(
                    Identifier::new(self.ctx, "llvm.emit_c_interface"),
                    Attribute::unit(self.ctx),
                )],
            )?;
        }

        Ok(())
    }
}
