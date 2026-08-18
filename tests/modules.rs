mod common;

use std::path::PathBuf;

use common::compile_and_execute_project;
use rstest::rstest;

#[rstest]
#[case("examples/projects/modules", 37)]
#[case("examples/projects/multi_path_import", 15)]
#[case("examples/projects/import_dir", 22)]
#[case("examples/projects/import_all", 15)]
fn test_imports(#[case] path: PathBuf, #[case] expected: i64) {
    let result = compile_and_execute_project(&path);
    assert_eq!(result, expected);
}
