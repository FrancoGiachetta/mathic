use std::path::Path;

use ariadne::{Report, ReportBuilder, ReportKind};
use thiserror::Error;

use crate::{
    diagnostics::{ReportSpan, report},
    lowering::ir::types::MathicType,
    parser::Span,
};

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

    #[error("The struct initialization is missing some fields")]
    MissingStructFields { missing: String, span: Span },
}

fn get_report_span(error: &LoweringError, path: String) -> ReportSpan {
    match error {
        LoweringError::UndeclaredVariable { span, .. }
        | LoweringError::UndeclaredFunction { span, .. }
        | LoweringError::UndeclaredType { span }
        | LoweringError::DuplicateDeclaration { span, .. }
        | LoweringError::WrongArgumentCount { span, .. }
        | LoweringError::UnsupportedFeature { span, .. }
        | LoweringError::MismatchedType { span, .. }
        | LoweringError::MismatchedReturnType { span, .. }
        | LoweringError::UndeclaredStructField { span, .. }
        | LoweringError::MissingStructFields { span, .. } => ReportSpan { path, span: *span },
    }
}

pub fn format_lowering_error<'err>(
    file_path: &'err Path,
    error: &LoweringError,
) -> ReportBuilder<'err, ReportSpan> {
    let path = file_path.display().to_string();
    let error_type = "Semantic Error";
    let msg = error.to_string();
    let report_span = get_report_span(error, path);

    match error {
        LoweringError::UndeclaredVariable { name, .. } => {
            report!(
                "S001",
                "Semantic Error",
                msg,
                report_span,
                format!("declare '{}' with 'let' before using it", name)
            )
        }
        LoweringError::DuplicateDeclaration { name, .. } => report!(
            "S002",
            "Semantic Error",
            msg,
            report_span,
            format!("'{}' is already declared in this scope", name)
        ),
        LoweringError::WrongArgumentCount { expected, .. } => {
            report!(
                "S003",
                error_type,
                msg,
                report_span,
                format!("expected {} argument(s)", expected)
            )
        }
        LoweringError::UndeclaredFunction { .. } => report!(
            "S004",
            "Semantic Error",
            msg,
            report_span,
            "declare the function before calling it".to_string()
        ),
        LoweringError::UndeclaredType { .. } => {
            report!(
                "S005",
                error_type,
                msg,
                report_span,
                "declare it using it".to_string()
            )
        }
        LoweringError::UnsupportedFeature { feature, .. } => {
            report!(
                "S006",
                "Semantic Error",
                msg,
                report_span,
                format!("{} is not yet implemented", feature)
            )
        }
        LoweringError::MismatchedType {
            found, expected, ..
        } => create_mismatched_type_report(expected, found, report_span),
        LoweringError::MismatchedReturnType {
            expected, found, ..
        } => report!(
            "S008",
            "Semantic Error",
            msg,
            report_span,
            format!(
                "function expects return type '{}', found '{}'",
                expected, found
            )
        ),
        LoweringError::UndeclaredStructField { .. } => {
            report!(
                "S009",
                "Semantic Error",
                msg,
                report_span,
                "check struct declaration".to_string()
            )
        }
        LoweringError::MissingStructFields { missing, .. } => report!(
            "S010",
            error_type,
            msg,
            report_span,
            format!("initialize the missing fields: {missing}")
        ),
    }
}

fn create_mismatched_type_report<'err>(
    expected: &MathicType,
    found: &MathicType,
    report_span: ReportSpan,
) -> ReportBuilder<'err, ReportSpan> {
    match (expected, found) {
        (MathicType::Array { length: elen, .. }, MathicType::Array { length: flen, .. })
            if elen != flen =>
        {
            report!(
                "S007",
                "Semantic Error",
                "Mismatched type",
                report_span,
                format!(
                    "expected an array {} elements, found one with {}",
                    elen, flen
                ),
                format!("expected: {expected}, got {found}")
            )
        }
        _ => report!(
            "S007",
            "Semantic Error",
            "Mismatched type",
            report_span,
            format!("expected: {expected}, got {found}")
        ),
    }
}
