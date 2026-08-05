use llvm_sys::{
    error::{LLVMDisposeErrorMessage, LLVMGetErrorMessage},
    orc2::{
        LLVMOrcExecutorAddress,
        lljit::{LLVMOrcDisposeLLJIT, LLVMOrcLLJITLookup, LLVMOrcLLJITRef},
    },
};
use melior::ir::Module;
use std::mem;

use crate::{
    codegen::compiler_helper::debugging, compiler::CompilerOpts, diagnostics::CodegenError,
    executor::MathicExecutor, ffi,
};

/// A wrapper over melior's ExecutionEngine.
pub struct MathicJITExecutor {
    engine: LLVMOrcLLJITRef,
}

impl MathicJITExecutor {
    // Creates the LLJIT
    pub fn new(modules: &[Module], compiler_options: CompilerOpts) -> Result<Self, CodegenError> {
        let engine = ffi::llvm::create_llvm_jit(
            modules,
            compiler_options.opt_lvl.into(),
            compiler_options.dump_llvmir,
        )?;
        let executor = Self { engine };

        debugging::debug_utils_runtime::setup(|sym| executor.lookup_symbol(sym));

        Ok(executor)
    }
}

impl MathicExecutor for MathicJITExecutor {
    /// Executes a function.
    ///
    /// Given a symbol_name (the name of the function to execute) the engine looks
    /// for the associated function and executes it.
    fn call_function(&self, symbol_name: &str) -> Result<i64, CodegenError> {
        let func: fn() -> i64 = unsafe {
            mem::transmute(
                self.lookup_symbol(&format!("mathic__{}", symbol_name))
                    .ok_or(CodegenError::LLVMError(format!(
                        "symbol 'mathic__{symbol_name}' not found"
                    )))?,
            )
        };

        Ok(func())
    }

    /// Returns a pointer associated to the given symbol name.
    ///
    /// if the symbol was registered, the engine will find it and return the
    /// associated pointer, otherwise it will return None.
    fn lookup_symbol(&self, symbol_name: &str) -> Option<*mut ()> {
        let c_name = std::ffi::CString::new(symbol_name).ok()?;

        let mut sym_addr: LLVMOrcExecutorAddress = 0;

        let err = unsafe { LLVMOrcLLJITLookup(self.engine, &mut sym_addr, c_name.as_ptr()) };

        if !err.is_null() {
            unsafe { LLVMDisposeErrorMessage(LLVMGetErrorMessage(err)) };
            return None;
        }

        Some(sym_addr as *mut ())
    }
}

impl Drop for MathicJITExecutor {
    fn drop(&mut self) {
        unsafe {
            LLVMOrcDisposeLLJIT(self.engine);
        }
    }
}
