use melior::{ExecutionEngine, ir::Module};
use std::ffi::c_void;

use crate::error::{MathicError, Result};

pub struct MathicExecutor {
    engine: ExecutionEngine,
}

impl MathicExecutor {
    pub fn new(module: &Module, optimization_level: u32) -> Result<Self> {
        let engine = ExecutionEngine::new(module, optimization_level, &[], false)?;

        Ok(Self { engine })
    }

    pub fn execute_main(&self) -> Result<i64> {
        // For invoke_packed, the result pointer should be the last argument
        let mut result: i64 = 0;
        let args: &mut [*mut c_void] = &mut [
            &mut result as *mut i64 as *mut c_void, // result pointer
        ];

        unsafe {
            self.engine.invoke_packed("main", args)?;
        }

        Ok(result)
    }

    pub fn lookup_symbol(&self, symbol_name: &str) -> Option<*mut c_void> {
        match self.engine.lookup(symbol_name) {
            Ok(ptr) => Some(ptr),
            Err(_) => None,
        }
    }
}
