use std::{fs, ops::Range, path::Path};

use ariadne::{Color, FnCache, Label, Report, ReportKind};

use super::error::{LexError, ParseError, SyntaxError};

#[derive(Clone)]
pub struct ReportSpan {
    pub path: String,
    pub span: Range<usize>,
}

impl ariadne::Span for ReportSpan {
    type SourceId = String;

    fn source(&self) -> &Self::SourceId {
        &self.path
    }

    fn start(&self) -> usize {
        self.span.start
    }

    fn end(&self) -> usize {
        self.span.end
    }
}

pub fn format_error(file_path: &Path, error: &ParseError) {
    let path = file_path.display().to_string();

    let report = match error {
        ParseError::Lexical(lex_error, span) => {
            let report_span = ReportSpan {
                path: path.clone(),
                span: span.start..span.end,
            };
            match lex_error {
                LexError::TokenError => Report::build(ReportKind::Error, report_span.clone())
                    .with_code("L001")
                    .with_message("lexical error")
                    .with_label(
                        Label::new(report_span.clone())
                            .with_color(Color::Red)
                            .with_message("unknown token"),
                    )
                    .finish(),
                LexError::InvalidCharacter(c) => {
                    Report::build(ReportKind::Error, report_span.clone())
                        .with_code("L002")
                        .with_message("lexical error")
                        .with_label(
                            Label::new(report_span)
                                .with_color(Color::Red)
                                .with_message(format!("invalid character: '{}'", c)),
                        )
                        .finish()
                }
                LexError::UnterminatedString => {
                    Report::build(ReportKind::Error, report_span.clone())
                        .with_code("L003")
                        .with_message("lexical error")
                        .with_label(
                            Label::new(report_span)
                                .with_color(Color::Red)
                                .with_message("unterminated string"),
                        )
                        .finish()
                }
                LexError::UnterminatedComment => {
                    Report::build(ReportKind::Error, report_span.clone())
                        .with_code("L004")
                        .with_message("lexical error")
                        .with_label(
                            Label::new(report_span)
                                .with_color(Color::Red)
                                .with_message("unterminated comment"),
                        )
                        .finish()
                }
                LexError::InvalidNumber(n) => Report::build(ReportKind::Error, report_span.clone())
                    .with_code("L005")
                    .with_message("lexical error")
                    .with_label(
                        Label::new(report_span)
                            .with_color(Color::Red)
                            .with_message(format!("invalid number: {}", n)),
                    )
                    .finish(),
            }
        }
        ParseError::Syntax(syntax_error) => {
            let (code, msg, span, help) = match syntax_error {
                SyntaxError::UnexpectedToken { found, expected } => {
                    let help_msg = match expected.as_str() {
                        "statement" => {
                            "valid statements include: function declarations, if/while/for, return, or blocks {}"
                        }
                        "identifier" => {
                            "only variable or function names can be called, e.g., 'foo()' or 'bar()'"
                        }
                        "expression" => {
                            "expressions can be: numbers, booleans, identifiers, or parenthesized expressions"
                        }
                        other => other,
                    };
                    (
                        "E001",
                        format!("expected {}, found '{}'", expected, found.lexeme),
                        &found.span,
                        help_msg.to_string(),
                    )
                }
                SyntaxError::UnexpectedEnd { expected, span } => {
                    let help_msg = match expected.as_str() {
                        "statement" => {
                            "file ended unexpectedly - check for missing closing braces '}'"
                        }
                        "identifier" => "expected an identifier (variable or function name)",

                        "expression" => {
                            "expression is incomplete - check for missing operands or operators"
                        }

                        other => other,
                    };
                    (
                        "E002",
                        format!("expected \"{}\", found end of file", expected),
                        span,
                        help_msg.to_string(),
                    )
                }
                SyntaxError::MissingToken { expected, span } => (
                    "E003",
                    format!("expected '{}'", expected),
                    span,
                    format!("add '{}' here to complete the syntax", expected),
                ),
            };

            let report_span = ReportSpan {
                path: path.clone(),
                span: span.start..span.end,
            };

            Report::build(ReportKind::Error, report_span.clone())
                .with_code(code)
                .with_message("syntax error")
                .with_label(
                    Label::new(report_span)
                        .with_color(Color::Red)
                        .with_message(msg),
                )
                .with_help(help)
                .finish()
        }
    };

    report
        .eprint(FnCache::new(|p: &String| fs::read_to_string(p)))
        .unwrap();
}
