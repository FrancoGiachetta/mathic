use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum TokenType {
    // Literals
    Number,
    String,
    True,
    False,
    // Keywords
    Struct,
    Let,
    If,
    Else,
    For,
    While,
    Sym,
    Def,
    Return,
    // Rest
    Identifier,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Equal,
    EqualEqual,
    NotEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Plus,
    Minus,
    Star,
    Slash,
    Not,
    SemiColon,
    Colon,
    Eof,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub r#type: TokenType,
    pub lexeme: Option<String>,
    pub line: u32,
    pub start: u16,
    pub end: u16,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Token:\n\ttype -> {:?}\n\tline -> {}\n\tstart -> {}\n\tend -> {}",
            self.r#type, self.line, self.start, self.end
        )
    }
}
