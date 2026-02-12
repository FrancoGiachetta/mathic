use std::num::ParseIntError;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum CodegenError {
    #[error("Missing main function")]
    MissingMainFunction,
    #[error("Invalid operation: {0}")]
    InvalidOperation(String),
    #[error(transparent)]
    MeliorError(#[from] melior::Error),
    #[error("LLVM error: {0}")]
    LLVMError(String),
    #[error(transparent)]
    ParseIntError(#[from] ParseIntError),
    #[error("Couldn't not parse attribute")]
    ParseAttributeError,
    #[error("{0}")]
    Custom(String),
}
