use std::{
    fmt::{Display, Formatter},
    path::Path,
};

use ariadne::{Report, ReportBuilder, ReportKind};
use thiserror::Error;

use crate::{
    diagnostics::ReportSpan,
    parser::{
        lexer::{Span, SpannedToken},
        token::Token,
    },
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

pub fn format_parse_error<'err>(
    file_path: &Path,
    error: &ParseError,
) -> ReportBuilder<'err, ReportSpan> {
    let path = file_path.display().to_string();

    let (code, msg, span, help) = match error {
        ParseError::Lexical(lex_error, span) => {
            let (code, msg) = match lex_error {
                LexError::TokenError => ("L001", "unknown token".to_string()),
                LexError::InvalidCharacter(c) => ("L002", format!("invalid character: '{}'", c)),
                LexError::UnterminatedString => ("L003", "unterminated string".to_string()),
                LexError::UnterminatedComment => ("L004", "unterminated comment".to_string()),
                LexError::InvalidNumber(n) => ("L005", format!("invalid number: {}", n)),
            };
            (code, msg, span.clone(), "Lexical Error".to_string())
        }
        ParseError::Syntax(syntax_error) => match syntax_error {
            SyntaxError::UnexpectedToken { found, expected } => (
                "E001",
                format!("expected {}, found '{}'", expected, found.lexeme),
                found.span.clone(),
                expected.help().to_string(),
            ),
            SyntaxError::UnexpectedEnd { span } => (
                "E002",
                "found an unexpected end of file".to_string(),
                span.clone(),
                String::new(),
            ),
            SyntaxError::MissingToken { expected, span } => (
                "E003",
                format!("expected '{}'", expected),
                span.clone(),
                format!("add '{}' here to complete the syntax", expected),
            ),
        },
    };

    let report_span = ReportSpan {
        path,
        span: span.start..span.end,
    };

    Report::build(ReportKind::Error, report_span.clone())
        .with_code(code)
        .with_message("Syntax Error")
        .with_label(
            ariadne::Label::new(report_span)
                .with_color(ariadne::Color::Red)
                .with_message(msg),
        )
        .with_help(help)
}
