use std::{fs, ops::Range, path::Path};

use ariadne::{Color, FnCache, Label, Report, ReportKind};

use super::error::{LexError, ParseError, SemanticError, SyntaxError};

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
                    .with_message("lexical error")
                    .with_label(
                        Label::new(report_span.clone())
                            .with_color(Color::Red)
                            .with_message("unknown token"),
                    )
                    .finish(),
                LexError::InvalidCharacter(c) => {
                    Report::build(ReportKind::Error, report_span.clone())
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
                        .with_message("lexical error")
                        .with_label(
                            Label::new(report_span)
                                .with_color(Color::Red)
                                .with_message("unterminated comment"),
                        )
                        .finish()
                }
                LexError::InvalidNumber(n) => Report::build(ReportKind::Error, report_span.clone())
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
            let (msg, span) = match syntax_error {
                SyntaxError::UnexpectedToken { found, span } => {
                    (format!("unexpected token: '{}'", found), span)
                }
                SyntaxError::UnexpectedEnd { span } => {
                    ("unexpected end of input".to_string(), span)
                }
                SyntaxError::MissingToken { expected, span } => {
                    (format!("expected: {}", expected), span)
                }
                SyntaxError::InvalidExpression { context, span } => {
                    (format!("invalid expression: {}", context), span)
                }
                SyntaxError::InvalidFunctionDefinition { span } => {
                    ("invalid function definition".to_string(), span)
                }
                SyntaxError::InvalidParameter { reason, span } => {
                    (format!("invalid parameter: {}", reason), span)
                }
                SyntaxError::InvalidTypeAnnotation { found, span } => {
                    (format!("invalid type annotation: {}", found), span)
                }
            };

            let report_span = ReportSpan {
                path: path.clone(),
                span: span.start..span.end,
            };

            Report::build(ReportKind::Error, report_span.clone())
                .with_message("syntax error")
                .with_label(
                    Label::new(report_span)
                        .with_color(Color::Red)
                        .with_message(msg),
                )
                .finish()
        }
        ParseError::Semantic(semantic_error) => {
            let (msg, span) = match semantic_error {
                SemanticError::DuplicateParameterName { name, span } => {
                    (format!("duplicate parameter name: '{}'", name), span)
                }
                SemanticError::DuplicateFunction { name, span } => {
                    (format!("duplicate function name: '{}'", name), span)
                }
                SemanticError::InvalidAssignment { target, span } => {
                    (format!("invalid assignment target: '{}'", target), span)
                }
                SemanticError::InvalidReturn { span } => {
                    ("invalid return statement".to_string(), span)
                }
                SemanticError::UnknownType { name, span } => {
                    (format!("unknown type: '{}'", name), span)
                }
            };

            let report_span = ReportSpan {
                path: path.clone(),
                span: span.start..span.end,
            };

            Report::build(ReportKind::Error, report_span.clone())
                .with_message("semantic error")
                .with_label(
                    Label::new(report_span)
                        .with_color(Color::Yellow)
                        .with_message(msg),
                )
                .finish()
        }
    };

    report
        .eprint(FnCache::new(|p: &String| fs::read_to_string(p)))
        .unwrap();
}
