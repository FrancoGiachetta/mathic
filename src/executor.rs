use melior::{ExecutionEngine, ir::Module};

use crate::{MathicResult, codegen::error::CodegenError, compiler::OptLvl};

pub struct MathicExecutor {
    engine: ExecutionEngine,
}

impl MathicExecutor {
    pub fn new(module: &Module, opt_lvl: OptLvl) -> MathicResult<Self> {
        let engine = ExecutionEngine::new(module, opt_lvl as usize, &[], false);

        Ok(Self { engine })
    }

    pub fn execute_main(&self) -> Result<i64, CodegenError> {
        // For invoke_packed, the result pointer should be the last argument
        let mut result: i64 = 0;
        let args: &mut [*mut ()] = &mut [
            &mut result as *mut i64 as *mut (), // result pointer
        ];

        unsafe {
            self.engine.invoke_packed("main", args)?;
        }

        Ok(result)
    }

    pub fn lookup_symbol(&self, symbol_name: &str) -> Option<*mut ()> {
        let ptr = self.engine.lookup(symbol_name);

        if ptr.is_null() { None } else { Some(ptr) }
    }
}
