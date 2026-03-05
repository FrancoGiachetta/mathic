mod common;

use std::path::PathBuf;

use common::compile_and_execute;
use rstest::rstest;

#[rstest]
#[case("tests/programs/functions/simple_function.mth", 7)]
#[case("tests/programs/functions/fibonacci.mth", 55)]
#[case("tests/programs/functions/factorial.mth", 120)]
#[case("tests/programs/functions/nested_calls.mth", 26)]
#[case("tests/programs/functions/forward_call.mth", 10)]
#[case("tests/programs/functions/forward_multiple_calls.mth", 8)]
#[case("tests/programs/functions/forward_mutual_recursion.mth", 1)]
fn test_functions(#[case] path: PathBuf, #[case] expected: i64) {
    let result = compile_and_execute(&path);
    assert_eq!(result, expected);
}
