mod common;

use std::path::PathBuf;

use common::compile_and_execute;
use rstest::rstest;

#[rstest]
#[case("tests/programs/variables/basic_declaration.mth", 42)]
#[case("tests/programs/variables/reassignment.mth", 20)]
#[case("tests/programs/variables/multiple_variables.mth", 15)]
#[case("tests/programs/variables/variable_increment.mth", 20)]
fn test_variables(#[case] path: PathBuf, #[case] expected: i64) {
    let result = compile_and_execute(&path);
    assert_eq!(result, expected);
}
