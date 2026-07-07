mod common;

use std::path::PathBuf;

use common::compile_and_execute;
use rstest::rstest;

#[rstest]
#[case("examples/symbolic/basic_sym.mth", 10)]
#[case("examples/symbolic/mul_add.mth", 30)]
#[case("examples/symbolic/nested.mth", 50)]
#[case("examples/symbolic/div.mth", 10)]
#[case("examples/symbolic/mix.mth", 101)]
#[case("examples/symbolic/mixed.mth", 66)]
fn test_symbolic(#[case] path: PathBuf, #[case] expected: i64) {
    let result = compile_and_execute(&path);
    assert_eq!(result, expected);
}
