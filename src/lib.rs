use crate::diagnostics::MathicError;

mod codegen;
pub mod compiler;
pub mod diagnostics;
pub mod executor;
mod ffi;
mod lowering;
mod parser;
mod types;

#[cfg(test)]
mod test_utils;

pub type MathicResult<T> = std::result::Result<T, MathicError>;
