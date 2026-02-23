use std::path::Path;

use ariadne::{Report, ReportBuilder};

use crate::error_reporter::ReportSpan;
use crate::lowering::error::LoweringError;

pub fn format_error<'err>(
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
        LoweringError::WrongArgumentCount {
            name,
            expected,
            span,
            ..
        } => (
            "S003",
            format!("provide {} argument(s) to '{}'", expected, name),
            span,
        ),
        LoweringError::UndefinedFunction { span, .. } => (
            "S004",
            "declare the function before calling it".to_string(),
            span,
        ),
        LoweringError::MissingReturn { span, .. } => (
            "S005",
            "add a return statement to the function".to_string(),
            span,
        ),
        LoweringError::UnsupportedFeature { span, feature } => {
            ("S006", format!("{} is not yet implemented", feature), span)
        }
    };

    let report_span = ReportSpan {
        path,
        span: span.start..span.end,
    };

    Report::build(ariadne::ReportKind::Error, report_span.clone())
        .with_code(code)
        .with_message("Semantic Error")
        .with_label(
            ariadne::Label::new(report_span)
                .with_color(ariadne::Color::Red)
                .with_message(msg),
        )
        .with_help(help)
}
