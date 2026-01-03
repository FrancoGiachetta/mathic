use logos::Logos;

use crate::parser::lexer::LexError;

#[derive(logos_display::Display, logos_display::Debug, Logos, PartialEq, Eq, Clone)]
#[logos( error = LexError, skip r"[ \t\r\n\f]+", skip r"//[^/][^\n]*")]
pub enum Token {
    // Single char
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,
    #[token("[")]
    LSquareBracket,
    #[token("]")]
    RSquareBracket,
    #[token("<")]
    Less,
    #[token(">")]
    Grater,
    #[token("=")]
    Eq,
    #[token("!")]
    Bang,
    #[token(":")]
    Colon,
    #[token(".")]
    Dot,
    #[token(",")]
    Comma,
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Star,
    #[token("/")]
    Slash,

    // Multi char
    #[token("=<")]
    EqLess,
    #[token("=>")]
    EqGrater,
    #[token("==")]
    EqEq,
    #[token("!=")]
    BangEq,

    // Keywords
    #[token("if")]
    If,
    #[token("else")]
    Else,
    #[token("while")]
    While,
    #[token("for")]
    For,
    #[token("struct")]
    Struct,
    #[token("df")]
    Df,
    #[token("let")]
    Let,
    #[token("return")]
    Return,
    #[token("and")]
    And,
    #[token("or")]
    Or,

    // Literals
    #[regex(r#""([^"])""#)]
    Str,
    #[regex(r"(?:0|[1-9]\d+)(?:\.\d+)?")]
    Num,
    #[regex(r"[[:alpha:]][[:alnum:]]*")]
    Ident,
}
