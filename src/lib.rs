use crate::error::MathicError;

mod codegen;
pub mod compiler;
pub mod error;
pub mod error_reporter;
pub mod executor;
mod ffi;
mod lowering;
mod parser;

#[cfg(test)]
mod test_utils;

pub type MathicResult<T> = std::result::Result<T, MathicError>;
