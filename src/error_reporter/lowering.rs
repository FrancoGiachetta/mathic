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
    let (code, help) = match error {
        LoweringError::UndefinedVariable { name, .. } => (
            "S001",
            format!("declare '{}' with 'let' before using it", name),
        ),
        LoweringError::DuplicateDeclaration { name, .. } => (
            "S002",
            format!("'{}' is already declared in this scope", name),
        ),
        LoweringError::WrongArgumentCount { name, expected, .. } => (
            "S003",
            format!("provide {} argument(s) to '{}'", expected, name),
        ),
        LoweringError::UndefinedFunction { .. } => {
            ("S004", "declare the function before calling it".to_string())
        }
        LoweringError::MissingReturn { .. } => {
            ("S005", "add a return statement to the function".to_string())
        }
        LoweringError::UnsupportedFeature { .. } => {
            ("S006", "this feature is not yet implemented".to_string())
        }
    };

    let span = &error.span();
    let report_span = ReportSpan {
        path,
        span: span.start..span.end,
    };

    Report::build(ariadne::ReportKind::Error, report_span.clone())
        .with_code(code)
        .with_message("lowering error")
        .with_label(
            ariadne::Label::new(report_span)
                .with_color(ariadne::Color::Red)
                .with_message(msg),
        )
        .with_help(help)
}
