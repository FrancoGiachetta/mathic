use std::{fs, ops::Range, path::Path};

use ariadne::FnCache;
use thiserror::Error;

pub mod codegen;
pub mod lowering;
pub mod parse;

pub use codegen::CodegenError;
pub use lowering::LoweringError;
pub use parse::{LexError, ParseError, SyntaxError};

use crate::diagnostics::{lowering::format_lowering_error, parse::format_parse_error};

#[derive(Debug, Error)]
pub enum MathicError {
    #[error(transparent)]
    Codegen(#[from] CodegenError),
    #[error(transparent)]
    Parse(#[from] ParseError),
    #[error(transparent)]
    Lowering(#[from] LoweringError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

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

pub fn format_error(file_path: &Path, error: &MathicError) {
    let report = match error {
        MathicError::Parse(parse_error) => format_parse_error(file_path, parse_error),
        MathicError::Lowering(lowering_error) => format_lowering_error(file_path, lowering_error),
        MathicError::Codegen(codegen_error) => unimplemented!("{:?}", codegen_error),
        MathicError::Io(_) => unimplemented!(),
    };

    report
        .finish()
        .eprint(FnCache::new(|p: &String| fs::read_to_string(p)))
        .unwrap();
}
