use crate::error::MathicError;

mod codegen;
pub mod compiler;
pub mod error;
pub mod executor;
mod parser;

pub type Result<T> = std::result::Result<T, MathicError>;
