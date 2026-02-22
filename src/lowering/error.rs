use thiserror::Error;

use crate::parser::lexer::Span;

#[derive(Debug, Error)]
pub enum LoweringError {
    #[error("Undefined variable '{name}'")]
    UndefinedVariable { name: String, span: Span },

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

impl LoweringError {
    pub fn span(&self) -> &Span {
        match self {
            LoweringError::UndefinedVariable { span, .. } => span,
            LoweringError::DuplicateDeclaration { span, .. } => span,
            LoweringError::WrongArgumentCount { span, .. } => span,
            LoweringError::UndefinedFunction { span, .. } => span,
            LoweringError::MissingReturn { span, .. } => span,
            LoweringError::UnsupportedFeature { span, .. } => span,
        }
    }

    pub fn help(&self) -> String {
        match self {
            LoweringError::UndefinedVariable { name, .. } => {
                format!("declare '{}' with 'let' before using it", name)
            }
            LoweringError::DuplicateDeclaration { name, .. } => {
                format!("'{}' is already declared in this scope", name)
            }
            LoweringError::WrongArgumentCount { name, expected, .. } => {
                format!("provide {} argument(s) to '{}'", expected, name)
            }
            LoweringError::UndefinedFunction { .. } => {
                "declare the function before calling it".to_string()
            }
            LoweringError::MissingReturn { .. } => {
                "add a return statement to the function".to_string()
            }
            LoweringError::UnsupportedFeature { .. } => {
                "this feature is not yet implemented".to_string()
            }
        }
    }
}
