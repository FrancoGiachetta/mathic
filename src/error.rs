use crate::codegen::error::CodegenError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MathicError {
    #[error(transparent)]
    Codegen(#[from] CodegenError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
