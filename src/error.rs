use crate::{codegen::error::CodegenError, parser::error::ParseError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MathicError {
    #[error(transparent)]
    Codegen(#[from] CodegenError),
    #[error(transparent)]
    Parse(#[from] ParseError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
