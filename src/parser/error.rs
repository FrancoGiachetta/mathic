use std::fmt::{Display, Formatter};

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
pub enum ExpectedToken {
    Statement,
    Identifier,
    Expression,
    Token(Token),
    Custom(String),
}

impl Display for ExpectedToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ExpectedToken::Statement => write!(f, "statement"),
            ExpectedToken::Expression => write!(f, "expression"),
            ExpectedToken::Identifier => write!(f, "identifier"),
            ExpectedToken::Token(t) => write!(f, "{}", t),
            ExpectedToken::Custom(s) => write!(f, "{}", s),
        }
    }
}

impl ExpectedToken {
    pub fn help(&self) -> &'static str {
        match self {
            ExpectedToken::Statement => {
                "valid statements include: function declarations, if/while/for, return, or blocks"
            }
            ExpectedToken::Identifier => {
                "only variable or function names can be called, e.g., 'foo()' or 'bar()'"
            }
            ExpectedToken::Expression => {
                "expressions can be: numbers, booleans, identifiers, or parenthesized expressions"
            }
            ExpectedToken::Token(_) | ExpectedToken::Custom(_) => "",
        }
    }
}

#[derive(Debug, Clone)]
pub enum SyntaxError {
    UnexpectedToken {
        found: FoundToken,
        expected: ExpectedToken,
    },
    UnexpectedEnd {
        span: Span,
    },
    MissingToken {
        expected: Token,
        span: Span,
    },
}

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
