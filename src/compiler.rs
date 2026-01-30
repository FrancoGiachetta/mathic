use melior::ir::Module;
use std::{fs, path::Path};

use crate::{
    codegen::MathicCodeGen,
    error::{MathicError, Result},
    parser::MathicParser,
};

pub struct Compiler;

impl Compiler {
    pub fn compile(file_path: &Path, optimization_level: u32) -> Result<Module> {
        // Read source file
        let source = fs::read_to_string(file_path)?;

        // Parse the source code
        let parser = MathicParser::new(&source);
        let ast = parser.parse()?;

        // Generate MLIR code
        let mut codegen = MathicCodeGen::new()?;
        let module = codegen.generate_module(&ast)?;

        Ok(module)
    }
}
