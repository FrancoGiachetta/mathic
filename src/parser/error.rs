use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("[line: {1}, column: {2}] Expected '{0}'")]
    UnexpectedToken(u32, u16, Box<str>),
}
