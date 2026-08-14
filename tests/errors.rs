mod common;

use std::path::{Path, PathBuf};

use mathic::{
    MathicError,
    compiler::{CompilerOpts, MathicCompiler},
    diagnostics::{CompilationError, LoweringError, ParseError},
};
use rstest::rstest;

fn compile_file(compiler: &MathicCompiler, file_path: &Path) -> Result<(), MathicError> {
    compiler
        .compile_path(file_path, CompilerOpts::default())
        .map(|_| ())
}

fn compile_project(compiler: &MathicCompiler, src_root: &Path) -> Result<(), MathicError> {
    compiler
        .compile_project(src_root, CompilerOpts::default())
        .map(|_| ())
}

/// Asserts that compilation failed and that the reported errors match `check`.
fn assert_compilation_failed(
    compiler: &MathicCompiler,
    result: Result<(), MathicError>,
    check: impl FnOnce(&[(PathBuf, CompilationError)]) -> bool,
) {
    assert!(
        matches!(result, Err(MathicError::CompilationFailed)),
        "expected Err(CompilationFailed), got {result:?}"
    );
    assert!(
        compiler.diagnostics().has_errors().unwrap(),
        "diagnostics should report errors"
    );

    let ok = compiler
        .diagnostics()
        .with_errors(check)
        .expect("diagnostics lock poisoned");
    assert!(ok, "reported errors did not match the expectation");
}

/// Asserts that compilation failed with a single error matching `check`.
fn assert_single_error(
    compiler: &MathicCompiler,
    result: Result<(), MathicError>,
    check: impl Fn(&CompilationError) -> bool,
) {
    assert_compilation_failed(compiler, result, |errors| {
        errors.len() == 1 && check(&errors[0].1)
    });
}

#[rstest]
#[case(
    "tests/fixtures/errors/syntax_error_missing_expression.mth",
    |error: &CompilationError| matches!(error, CompilationError::Parse(ParseError::Syntax(_)))
)]
#[case(
    "tests/fixtures/errors/syntax_error_missing_identifier.mth",
    |error: &CompilationError| matches!(error, CompilationError::Parse(ParseError::Syntax(_)))
)]
#[case(
    "tests/fixtures/errors/semantic_error_mismatched_type.mth",
    |error: &CompilationError| {
        matches!(error, CompilationError::Lowering(LoweringError::MismatchedType { .. }))
    }
)]
#[case(
    "tests/fixtures/errors/control_flow_error_if_non_boolean_condition.mth",
    |error: &CompilationError| {
        matches!(error, CompilationError::Lowering(LoweringError::MismatchedType { .. }))
    }
)]
#[case(
    "tests/fixtures/errors/control_flow_error_while_non_boolean_condition.mth",
    |error: &CompilationError| {
        matches!(error, CompilationError::Lowering(LoweringError::MismatchedType { .. }))
    }
)]
#[case(
    "tests/fixtures/errors/struct_error_field_type_mismatch.mth",
    |error: &CompilationError| {
        matches!(error, CompilationError::Lowering(LoweringError::MismatchedType { .. }))
    }
)]
#[case(
    "tests/fixtures/errors/symbolic_error_expr_non_numeric_parameter.mth",
    |error: &CompilationError| {
        matches!(error, CompilationError::Lowering(LoweringError::MismatchedType { .. }))
    }
)]
#[case(
    "tests/fixtures/errors/struct_error_undeclared_field.mth",
    |error: &CompilationError| {
        matches!(error, CompilationError::Lowering(LoweringError::UndeclaredStructField { .. }))
    }
)]
#[case(
    "tests/fixtures/errors/struct_error_undeclared_field_access.mth",
    |error: &CompilationError| {
        matches!(error, CompilationError::Lowering(LoweringError::UndeclaredStructField { .. }))
    }
)]
#[case(
    "tests/fixtures/errors/semantic_error_undeclared_variable.mth",
    |error: &CompilationError| {
        matches!(error, CompilationError::Lowering(LoweringError::UndeclaredVariable { .. }))
    }
)]
#[case(
    "tests/fixtures/errors/semantic_error_duplicate_declaration.mth",
    |error: &CompilationError| {
        matches!(error, CompilationError::Lowering(LoweringError::DuplicateDeclaration { .. }))
    }
)]
#[case(
    "tests/fixtures/errors/semantic_error_undeclared_function.mth",
    |error: &CompilationError| {
        matches!(error, CompilationError::Lowering(LoweringError::UndeclaredFunction { .. }))
    }
)]
#[case(
    "tests/fixtures/errors/semantic_error_wrong_argument_count.mth",
    |error: &CompilationError| {
        matches!(error, CompilationError::Lowering(LoweringError::WrongArgumentCount { .. }))
    }
)]
#[case(
    "tests/fixtures/errors/semantic_error_mismatched_return_type.mth",
    |error: &CompilationError| {
        matches!(error, CompilationError::Lowering(LoweringError::MismatchedReturnType { .. }))
    }
)]
#[case(
    "tests/fixtures/errors/semantic_error_undeclared_type.mth",
    |error: &CompilationError| {
        matches!(error, CompilationError::Lowering(LoweringError::UndeclaredType { .. }))
    }
)]
#[case(
    "tests/fixtures/errors/struct_error_missing_fields.mth",
    |error: &CompilationError| {
        matches!(error, CompilationError::Lowering(LoweringError::MissingStructFields { .. }))
    }
)]
#[case(
    "tests/fixtures/errors/symbolic_error_type_requires_type_parameter.mth",
    |error: &CompilationError| {
        matches!(error, CompilationError::Lowering(LoweringError::TypeRequiresTypeParameter { .. }))
    }
)]
fn source_error(#[case] path: PathBuf, #[case] check: fn(&CompilationError) -> bool) {
    let compiler = MathicCompiler::new().unwrap();
    assert_single_error(&compiler, compile_file(&compiler, &path), check);
}

#[rstest]
#[case(
    "tests/fixtures/error_parse",
    |error: &CompilationError| matches!(error, CompilationError::Parse(ParseError::Syntax(_)))
)]
#[case(
    "tests/fixtures/error_unresolved_import",
    |error: &CompilationError| {
        matches!(error, CompilationError::Lowering(LoweringError::UnResolvedPath { .. }))
    }
)]
#[case(
    "tests/fixtures/error_parse_gates_lowering/src",
    |error: &CompilationError| matches!(error, CompilationError::Parse(ParseError::Syntax(_)))
)]
fn project_error(#[case] src_root: PathBuf, #[case] check: fn(&CompilationError) -> bool) {
    let compiler = MathicCompiler::new().unwrap();
    assert_single_error(&compiler, compile_project(&compiler, &src_root), check);
}

#[rstest]
#[case(
    "tests/fixtures/error_lowering/src",
    |errors: &[(PathBuf, CompilationError)]| {
        errors.len() == 2
            && errors.iter().all(|(_, error)| {
                matches!(
                    error,
                    CompilationError::Lowering(LoweringError::UndeclaredVariable { .. })
                )
            })
    }
)]
fn project_lowering_errors_accumulate_across_files(
    #[case] src_root: PathBuf,
    #[case] check: fn(&[(PathBuf, CompilationError)]) -> bool,
) {
    let compiler = MathicCompiler::new().unwrap();
    assert_compilation_failed(&compiler, compile_project(&compiler, &src_root), check);
}
