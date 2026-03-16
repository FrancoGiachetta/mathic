mod common;

use std::path::PathBuf;

use common::compile_and_execute;
use rstest::rstest;

#[rstest]
#[case("examples/arithmetic/addition.mth", 8)]
#[case("examples/arithmetic/subtraction.mth", 6)]
#[case("examples/arithmetic/multiplication.mth", 42)]
#[case("examples/arithmetic/division.mth", 5)]
#[case("examples/arithmetic/order_of_operations.mth", 14)]
fn test_arithmetic(#[case] path: PathBuf, #[case] expected: i64) {
    let result = compile_and_execute(&path);
    assert_eq!(result, expected);
}
