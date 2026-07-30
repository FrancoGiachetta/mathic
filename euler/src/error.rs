use std::io;

use mathic::diagnostics::MathicError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum EulerError {
    #[error("main.mth was not found in src/")]
    MainFileNotFound,
    #[error(transparent)]
    MathicError(#[from] MathicError),
    #[error(transparent)]
    Io(#[from] io::Error),
}
