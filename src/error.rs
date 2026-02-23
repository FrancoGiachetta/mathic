use crate::codegen::error::CodegenError;
use crate::lowering::error::LoweringError;
use crate::parser::error::ParseError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MathicError {
    #[error(transparent)]
    Codegen(#[from] CodegenError),
    #[error(transparent)]
    Parse(#[from] ParseError),
    #[error(transparent)]
    Lowering(#[from] LoweringError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
