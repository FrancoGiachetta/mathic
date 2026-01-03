use std::ops::Range;

use crate::parser::lexer::LexError;

#[derive(Debug)]
pub enum ParseError {
    LexerError((LexError, Range<usize>)),
    UnexpectedToken(Box<str>),
    UnexpectedEnd,
}
