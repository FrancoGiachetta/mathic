mod codegen;
pub mod compiler;
pub mod diagnostics;
pub mod executor;
mod ffi;
mod lowering;
mod parser;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum MathicError {
    #[error("compilation failed")]
    CompilationFailed,
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("diagnostics lock poisoned: {0}")]
    LockPoisoned(String),
}

pub type MathicResult<T> = Result<T, MathicError>;
