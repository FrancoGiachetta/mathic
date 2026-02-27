use melior::{ExecutionEngine, ir::Module};

use crate::{MathicResult, compiler::OptLvl, diagnostics::CodegenError};

/// A wrapper over melior's ExecutionEngine.
pub struct MathicExecutor {
    engine: ExecutionEngine,
}

impl MathicExecutor {
    pub fn new(module: &Module, opt_lvl: OptLvl) -> MathicResult<Self> {
        let engine = ExecutionEngine::new(module, opt_lvl as usize, &[], false);

        Ok(Self { engine })
    }

    /// Executes a function.
    ///
    /// Given a symbol_name (the name of the function to execute) the engine looks
    /// for the associated function and executes it.
    pub fn call_function(&self, symbol_name: &str) -> Result<i64, CodegenError> {
        let mut result: i64 = 0;
        let args: &mut [*mut ()] = &mut [
            &mut result as *mut i64 as *mut (), // result pointer
        ];

        unsafe {
            self.engine
                .invoke_packed(&format!("mathic__{}", symbol_name), args)?;
        }

        Ok(result)
    }

    /// Returns a pointer associated to the given symbol name.
    ///
    /// if the symbol was registered, the engine will find it and return the
    /// associated pointer, otherwise it will return None.
    pub fn lookup_symbol(&self, symbol_name: &str) -> Option<*mut ()> {
        let ptr = self.engine.lookup(&format!("mathic__{}", symbol_name));

        if ptr.is_null() { None } else { Some(ptr) }
    }
}
