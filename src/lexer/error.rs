use thiserror::Error;

// (error description, line, column)
#[derive(Error, Debug)]
pub enum LexError {
    #[error("[line: {1}, column: {2}] '{0}' is not a valid lex")]
    UnexpectedLex(Box<str>, u32, u16),
    #[error(
        "[line: {1}, column: {2}] '{0}' is not a valid number, it can only be composed of digits"
    )]
    InvalidNumber(Box<str>, u32, u16),
    #[error("[line: {1}, column: {2}] {0}")]
    InvalidString(Box<str>, u32, u16),
    #[error("[line: {1}, column: {2}] '{0}' is not a valid character for an identifier")]
    InvalidIdentifier(Box<str>, u32, u16),
    #[error("[line: {0}] Unterminated muti-line comment, it should end with \"///\"")]
    InvalidComment(u32),
    #[error("[line: {0}] Index out of bounds")]
    UnableToSubString(u32),
}
