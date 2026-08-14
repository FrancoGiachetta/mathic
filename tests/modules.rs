mod common;

use std::path::PathBuf;

use common::compile_and_execute_project;
use rstest::rstest;

#[rstest]
#[case("examples/projects/modules", 252)]
#[case("examples/projects/modules_struct", 200)]
fn test_imports(#[case] path: PathBuf, #[case] expected: i64) {
    let result = compile_and_execute_project(&path);
    assert_eq!(result, expected);
}
