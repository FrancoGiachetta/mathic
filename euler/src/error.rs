use std::io;

use mathic::{MathicError, diagnostics::CodegenError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum EulerError {
    #[error("main.mth was not found")]
    MainFileNotFound,
    #[error(transparent)]
    MathicError(#[from] MathicError),
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    Toml(#[from] toml::ser::Error),
    #[error(transparent)]
    CodegenError(#[from] CodegenError),
}
