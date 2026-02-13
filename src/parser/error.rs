use thiserror::Error;

use crate::parser::{
    lexer::{Span, SpannedToken},
    token::Token,
};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum LexError {
    #[default]
    TokenError,
    InvalidCharacter(char),
    UnterminatedString,
    UnterminatedComment,
    InvalidNumber(String),
}

#[derive(Debug, Clone)]
pub enum SyntaxError {
    UnexpectedToken { found: String, span: Span },
    UnexpectedEnd { span: Span },
    MissingToken { expected: String, span: Span },
    InvalidExpression { context: String, span: Span },
    InvalidFunctionDefinition { span: Span },
    InvalidParameter { reason: String, span: Span },
    InvalidTypeAnnotation { found: String, span: Span },
}

#[derive(Debug, Clone)]
pub enum SemanticError {
    DuplicateParameterName { name: String, span: Span },
    DuplicateFunction { name: String, span: Span },
    InvalidAssignment { target: String, span: Span },
    InvalidReturn { span: Span },
    UnknownType { name: String, span: Span },
}

// ============ Unified ParseError ============
#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Lexical error")]
    Lexical(LexError, Span),
    #[error("Syntax error")]
    Syntax(SyntaxError),
    #[error("Semantic error")]
    Semantic(SemanticError),
}

impl LexError {
    pub fn from_lexer(lexer: &logos::Lexer<Token>) -> LexError {
        let source = lexer.source();
        let span = lexer.span();
        let slice = &source[span];

        let first = slice.chars().next().unwrap_or('\0');

        if slice.starts_with('"') {
            LexError::UnterminatedString
        } else if slice.starts_with("/*") {
            LexError::UnterminatedComment
        } else if first.is_numeric() {
            LexError::InvalidNumber(slice.to_string())
        } else if !first.is_alphanumeric() && !first.is_whitespace() {
            LexError::InvalidCharacter(first)
        } else {
            LexError::TokenError
        }
    }
}

impl From<SpannedToken<'_>> for SyntaxError {
    fn from(token: SpannedToken<'_>) -> Self {
        SyntaxError::UnexpectedToken {
            found: token.lexeme.to_string(),
            span: token.span,
        }
    }
}
