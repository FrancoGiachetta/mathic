mod common;

use std::path::PathBuf;

use common::compile_and_execute;
use rstest::rstest;

#[rstest]
#[case("examples/symbolic/div.mth", 10)]
#[case("examples/symbolic/mul_add_div.mth", 101)]
#[case("examples/symbolic/const_expr.mth", 66)]
#[case("examples/symbolic/var_expr.mth", 50)]
#[case("examples/symbolic/nested.mth", 2)]
#[case("examples/symbolic/multi_eval.mth", 30)]
#[case("examples/symbolic/big_expr.mth", 121)]
fn test_symbolic(#[case] path: PathBuf, #[case] expected: i64) {
    let result = compile_and_execute(&path);
    assert_eq!(result, expected);
}
