use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("[line: {1}, column: {2}] Expected '{0}'")]
    UnexpectedToken(u32, u16, Box<str>),
    #[error("Unexptect end of program while still parsing")]
    UnexpectedEnd,
}
