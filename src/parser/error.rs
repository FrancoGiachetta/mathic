use std::ops::Range;

use thiserror::Error;

use crate::parser::lexer::LexError;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Lexer error: {0}")]
    LexerError((LexError, Range<usize>)),
    #[error("Unexpected token: {0}")]
    UnexpectedToken(Box<str>),
    #[error("Unexpected end of input")]
    UnexpectedEnd,
}
