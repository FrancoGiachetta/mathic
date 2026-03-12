use std::path::Path;

use ariadne::{Report, ReportBuilder, ReportKind};
use thiserror::Error;

use crate::{diagnostics::ReportSpan, lowering::ir::types::MathicType, parser::Span};

#[derive(Debug, Error)]
pub enum LoweringError {
    #[error("Undeclared variable '{name}'")]
    UndeclaredVariable { name: String, span: Span },

    #[error("Undeclared function '{name}'")]
    UndeclaredFunction { name: String, span: Span },

    #[error("Undeclared type")]
    UndeclaredType { span: Span },

    #[error("Duplicate declaration of '{name}'")]
    DuplicateDeclaration { name: String, span: Span },

    #[error("Function '{name}' called with the wrong amount of arguments")]
    WrongArgumentCount {
        name: String,
        expected: usize,
        got: usize,
        span: Span,
    },

    #[error("Unsupported feature: {feature}")]
    UnsupportedFeature { feature: String, span: Span },

    #[error("Mismatched type")]
    MismatchedType {
        expected: MathicType,
        found: MathicType,
        span: Span,
    },

    #[error("Mismatched return type")]
    MismatchedReturnType {
        expected: MathicType,
        found: MathicType,
        span: Span,
    },

    #[error("The struct declaration does to have such field: {found}")]
    UndeclaredStructField { found: String, span: Span },

    #[error("the struct initialization is missing some fields")]
    MissingStructFields { missing: String, span: Span },
}

pub fn format_lowering_error<'err>(
    file_path: &'err Path,
    error: &LoweringError,
) -> ReportBuilder<'err, ReportSpan> {
    let path = file_path.display().to_string();

    let msg = error.to_string();
    let (code, help, span) = match error {
        LoweringError::UndeclaredVariable { name, span } => (
            "S001",
            format!("declare '{}' with 'let' before using it", name),
            span,
        ),
        LoweringError::DuplicateDeclaration { name, span } => (
            "S002",
            format!("'{}' is already declared in this scope", name),
            span,
        ),
        LoweringError::WrongArgumentCount { expected, span, .. } => {
            ("S003", format!("expected {} argument(s)", expected), span)
        }
        LoweringError::UndeclaredFunction { span, .. } => (
            "S004",
            "declare the function before calling it".to_string(),
            span,
        ),
        LoweringError::UndeclaredType { span, .. } => {
            ("S005", "declare it using it".to_string(), span)
        }
        LoweringError::UnsupportedFeature { span, feature } => {
            ("S006", format!("{} is not yet implemented", feature), span)
        }
        LoweringError::MismatchedType {
            span,
            found,
            expected,
        } => ("S007", format!("expected: {expected}, got {found}"), span),
        LoweringError::MismatchedReturnType {
            expected,
            found,
            span,
        } => (
            "S008",
            format!(
                "function expects return type '{}', found '{}'",
                expected, found
            ),
            span,
        ),
        LoweringError::UndeclaredStructField { span, .. } => {
            ("S009", format!("check struct declaration"), span)
        }
        LoweringError::MissingStructFields { missing, span } => (
            "S010",
            format!("initialize the missing fields: {missing}"),
            span,
        ),
    };

    let report_span = ReportSpan { path, span: *span };

    Report::build(ReportKind::Error, report_span.clone())
        .with_code(code)
        .with_message("Semantic Error")
        .with_label(
            ariadne::Label::new(report_span)
                .with_color(ariadne::Color::Red)
                .with_message(msg),
        )
        .with_help(help)
}
