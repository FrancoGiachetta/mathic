use thiserror::Error;

#[derive(Debug, Error)]
pub enum CodegenError {
    #[error("Missing main function")]
    MissingMainFunction,
    #[error(transparent)]
    MeliorError(#[from] melior::Error),
}
