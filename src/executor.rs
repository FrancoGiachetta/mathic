use crate::diagnostics::CodegenError;

pub mod jit;

pub trait MathicExecutor {
    /// Executes a function.
    ///
    /// Given a symbol_name (the name of the function to execute) the engine looks
    /// for the associated function and executes it.
    fn call_function(&self, symbol_name: &str) -> Result<i64, CodegenError>;

    /// Returns a pointer associated to the given symbol name.
    ///
    /// if the symbol was registered, the engine will find it and return the
    /// associated pointer, otherwise it will return None.
    fn lookup_symbol(&self, symbol_name: &str) -> Option<*mut ()>;
}
