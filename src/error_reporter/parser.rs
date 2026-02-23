use ariadne::ReportBuilder;

use crate::error_reporter::ReportSpan;
use crate::parser::error::{LexError, ParseError, SyntaxError};

pub fn format_error<'err>(
    file_path: &std::path::Path,
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

    ariadne::Report::build(ariadne::ReportKind::Error, report_span.clone())
        .with_code(code)
        .with_message("Syntax Error")
        .with_label(
            ariadne::Label::new(report_span)
                .with_color(ariadne::Color::Red)
                .with_message(msg),
        )
        .with_help(help)
}
