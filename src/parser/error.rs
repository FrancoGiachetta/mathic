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

/// Owned version of SpannedToken for error reporting
#[derive(Debug, Clone)]
pub struct FoundToken {
    pub lexeme: String,
    pub span: Span,
}

impl<'a> From<SpannedToken<'a>> for FoundToken {
    fn from(token: SpannedToken<'a>) -> Self {
        Self {
            lexeme: token.lexeme.to_string(),
            span: token.span,
        }
    }
}

#[derive(Debug, Clone)]
pub enum SyntaxError {
    UnexpectedToken { found: FoundToken, expected: String },
    UnexpectedEnd { span: Span },
    MissingToken { expected: Token, span: Span },
}

// ============ Unified ParseError ============
#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Lexical error")]
    Lexical(LexError, Span),
    #[error("Syntax error")]
    Syntax(SyntaxError),
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
