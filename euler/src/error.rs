use mathic::diagnostics::MathicError;
use thiserror::Error;

#[derive(Error)]
pub enum EulerError {
    #[error("main.mth was not found in src/")]
    MainFileNotFound,

    #[error(transparent)]
    MathicError(#[from] MathicError),
    #[error(transparent)]
    WalkDirError(#[from] walkdir),
}
