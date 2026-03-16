mod common;

use std::path::PathBuf;

use common::compile_and_execute;
use rstest::rstest;

#[rstest]
#[case("examples/control_flow/if_true.mth", 1)]
#[case("examples/control_flow/if_else_true.mth", 1)]
#[case("examples/control_flow/if_else_false.mth", 0)]
#[case("examples/control_flow/while_sum.mth", 55)]
#[case("examples/control_flow/for_sum.mth", 55)]
#[case("examples/control_flow/factorial.mth", 120)]
fn test_control_flow(#[case] path: PathBuf, #[case] expected: i64) {
    let result = compile_and_execute(&path);
    assert_eq!(result, expected);
}
