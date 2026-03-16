mod common;

use std::path::PathBuf;

use common::compile_and_execute;
use rstest::rstest;

#[rstest]
#[case("examples/functions/simple_function.mth", 7)]
#[case("examples/functions/fibonacci.mth", 55)]
#[case("examples/functions/factorial.mth", 120)]
#[case("examples/functions/nested_calls.mth", 26)]
#[case("examples/functions/forward_call.mth", 10)]
#[case("examples/functions/forward_multiple_calls.mth", 8)]
#[case("examples/functions/forward_mutual_recursion.mth", 1)]
fn test_functions(#[case] path: PathBuf, #[case] expected: i64) {
    let result = compile_and_execute(&path);
    assert_eq!(result, expected);
}
