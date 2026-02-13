use melior::{Context, ir::Module};

use crate::{MathicResult, codegen::error::CodegenError, error::MathicError, parser::ast::Program};

pub mod control_flow;
pub mod declaration;
pub mod error;
pub mod expression;
pub mod statement;
pub mod symbol_table;

pub use symbol_table::SymbolTable;

pub struct MathicCodeGen<'this, 'ctx> {
    module: &'this Module<'ctx>,
    // Tracks variables for the current function being compiled.
    // Reset when compiling a new global function.
    symbols: std::cell::RefCell<SymbolTable<'ctx, 'this>>,
}

impl<'this, 'ctx> MathicCodeGen<'this, 'ctx> {
    pub fn new(module: &'this Module<'ctx>) -> Self {
        Self {
            module,
            symbols: Default::default(),
        }
    }

    pub fn generate_module(&mut self, ctx: &'ctx Context, program: Program) -> MathicResult<()> {
        // Check if main function is present
        if !program.funcs.iter().any(|f| f.name == "main") {
            return Err(MathicError::Codegen(CodegenError::MissingMainFunction));
        }

        // TODO: Compile structs in the future

        for func in program.funcs {
            self.compile_function(ctx, func)?;

            self.symbols.replace(SymbolTable::new());
        }

        Ok(())
    }
}
