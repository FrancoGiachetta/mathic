use std::{
    fs,
    path::{Path, PathBuf},
    sync::Mutex,
};

use ariadne::{FnCache, ReportBuilder};
use thiserror::Error;

pub mod codegen;
pub mod lowering;
pub mod parse;

pub use codegen::CodegenError;
pub use lowering::LoweringError;
pub use parse::{LexError, ParseError, SyntaxError};

use crate::{
    MathicError, MathicResult,
    diagnostics::{lowering::format_lowering_error, parse::format_parse_error},
    parser::Span,
};

/// Errors produced by the compilation phases themselves.
#[derive(Debug, Error)]
pub enum CompilationError {
    #[error(transparent)]
    Codegen(#[from] CodegenError),
    #[error(transparent)]
    Parse(#[from] ParseError),
    #[error(transparent)]
    Lowering(#[from] LoweringError),
}

/// Accumulates compilation errors, one per file per phase.
#[derive(Debug, Default)]
pub struct DiagnosticsManager {
    errors: Mutex<Vec<(PathBuf, CompilationError)>>,
}

impl DiagnosticsManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn report(&self, file_path: PathBuf, error: CompilationError) -> MathicResult<()> {
        self.errors
            .lock()
            .map_err(|e| MathicError::LockPoisoned(e.to_string()))?
            .push((file_path, error));

        Ok(())
    }

    pub fn report_and_fail(&self, file_path: PathBuf, error: CompilationError) -> MathicError {
        let mut lock = match self.errors.lock() {
            Ok(l) => l,
            Err(e) => return MathicError::LockPoisoned(e.to_string()),
        };

        lock.push((file_path, error));

        MathicError::CompilationFailed
    }

    pub fn has_errors(&self) -> MathicResult<bool> {
        self.errors
            .lock()
            .map_err(|e| MathicError::LockPoisoned(e.to_string()))
            .map(|errors| !errors.is_empty())
    }

    pub fn error_count(&self) -> MathicResult<u32> {
        self.errors
            .lock()
            .map_err(|e| MathicError::LockPoisoned(e.to_string()))
            .map(|errors| errors.len() as u32)
    }

    pub fn clear(&self) -> MathicResult<()> {
        self.errors
            .lock()
            .map_err(|e| MathicError::LockPoisoned(e.to_string()))?
            .clear();

        Ok(())
    }

    pub fn print_all(&self) -> MathicResult<()> {
        let errors = self
            .errors
            .lock()
            .map_err(|e| MathicError::LockPoisoned(e.to_string()))?;

        for (file_path, error) in errors.iter() {
            format_error(file_path, error);
        }

        Ok(())
    }
}

#[derive(Clone)]
pub struct ReportSpan {
    pub path: String,
    pub span: Span,
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

fn eprint_report(report: ReportBuilder<ReportSpan>) {
    report
        .finish()
        .eprint(FnCache::new(|p: &String| fs::read_to_string(p)))
        .unwrap();
}

pub fn format_error(file_path: &Path, error: &CompilationError) {
    match error {
        CompilationError::Parse(parse_error) => {
            eprint_report(format_parse_error(file_path, parse_error))
        }
        CompilationError::Lowering(lowering_error) => {
            eprint_report(format_lowering_error(file_path, lowering_error))
        }
        CompilationError::Codegen(e) => eprintln!("{}: error: {}", file_path.display(), e),
    }
}
