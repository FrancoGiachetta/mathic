use std::{fs, path::PathBuf};

use ariadne::Source;
use melior::{
    Context,
    ir::{Location, Module},
};

use crate::{
    MathicResult,
    codegen::compiler_helper::CompilerHelper,
    diagnostics::{CodegenError, MathicError},
    lowering::ir::Ir,
    parser::Span,
};

pub mod compiler_helper;
pub mod function_ctx;
pub mod lvalue;
pub mod rvalue;

/// Struct that holds global infomation to the code generation.
///
/// ## Fields
///
/// **ctx**: MLIR Context, global to the whole compilation.
/// **module**: MLIR Module, where we store the generated mlir code.
/// **file_path**: the path to file being compiled.
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

    /// Returns a melior location.
    ///
    /// The location is relative to the file being compiled.
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

    /// Code generation entrypoint.
    ///
    /// Populates the module for a compile unit.
    pub fn generate_module(&self, program: &Ir, helper: &mut CompilerHelper) -> MathicResult<()> {
        // Check if main function is present
        if !program.functions.iter().any(|f| f.name == "main") {
            return Err(MathicError::Codegen(CodegenError::MissingMainFunction));
        }

        // TODO: Compile structs in the future

        for func in program.functions.iter() {
            self.compile_function(func, &[], helper)?;
        }

        Ok(())
    }
}
