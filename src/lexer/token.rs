#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum TokenType {
    // Literals
    Number,
    String,
    True,
    False,
    Type,
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
    Comma,
    Arrow,
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
