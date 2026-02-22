use thiserror::Error;

use crate::parser::lexer::Span;

#[derive(Debug, Error)]
pub enum LoweringError {
    #[error("Undeclared variable '{name}'")]
    UndeclaredVariable { name: String, span: Span },

    #[error("Duplicate declaration of '{name}'")]
    DuplicateDeclaration { name: String, span: Span },

    #[error("Function '{name}' called with {got} arguments, expected {expected}")]
    WrongArgumentCount {
        name: String,
        expected: usize,
        got: usize,
        span: Span,
    },

    #[error("Undefined function '{name}'")]
    UndefinedFunction { name: String, span: Span },

    #[error("Function '{name}' missing return statement")]
    MissingReturn { name: String, span: Span },

    #[error("Unsupported feature: {feature}")]
    UnsupportedFeature { feature: String, span: Span },
}
