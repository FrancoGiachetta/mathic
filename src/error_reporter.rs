use std::{fs, ops::Range, path::Path};

use ariadne::FnCache;

use crate::error::MathicError;

mod lowering;
mod parser;

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
        MathicError::Parse(parse_error) => parser::format_error(file_path, parse_error),
        MathicError::Lowering(lowering_error) => lowering::format_error(file_path, lowering_error),
        MathicError::Codegen(codegen_error) => unimplemented!("{:?}", codegen_error),
        MathicError::Io(_) => unimplemented!(),
    };

    report
        .finish()
        .eprint(FnCache::new(|p: &String| fs::read_to_string(p)))
        .unwrap();
}
