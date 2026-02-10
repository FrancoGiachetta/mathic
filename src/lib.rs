use crate::error::MathicError;

mod codegen;
pub mod compiler;
pub mod error;
pub mod executor;
mod ffi;
mod parser;

pub type MathicResult<T> = std::result::Result<T, MathicError>;
