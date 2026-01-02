use std::ops::Range;

use crate::parser::lexer::LexError;

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken((LexError, Range<usize>)),
    UnexpectedEnd,
}
