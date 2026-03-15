mod common;

use std::path::PathBuf;

use common::compile_and_execute;
use rstest::rstest;

#[rstest]
#[case("tests/programs/structs/basic_declaration.mth", 42)]
#[case("tests/programs/structs/local_struct.mth", 10)]
#[case("tests/programs/structs/field_assignment.mth", 42)]
#[case("tests/programs/structs/integrated_test.mth", 200)]
#[case("tests/programs/structs/return_struct_init.mth", 10)]
fn test_struct(#[case] path: PathBuf, #[case] expected: i64) {
    let result = compile_and_execute(&path);
    assert_eq!(result, expected);
}
