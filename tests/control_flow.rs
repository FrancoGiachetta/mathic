mod common;

#[test]
fn test_if_true() {
    let source = include_str!("programs/control_flow/if_true.mth");
    let result = common::compile_and_execute(source);
    assert_eq!(result, 1);
}

#[test]
fn test_if_else_true() {
    let source = include_str!("programs/control_flow/if_else_true.mth");
    let result = common::compile_and_execute(source);
    assert_eq!(result, 1);
}

#[test]
fn test_if_else_false() {
    let source = include_str!("programs/control_flow/if_else_false.mth");
    let result = common::compile_and_execute(source);
    assert_eq!(result, 0);
}

#[test]
fn test_while_sum() {
    let source = include_str!("programs/control_flow/while_sum.mth");
    let result = common::compile_and_execute(source);
    assert_eq!(result, 55);
}

#[test]
fn test_for_sum() {
    let source = include_str!("programs/control_flow/for_sum.mth");
    let result = common::compile_and_execute(source);
    assert_eq!(result, 55);
}

#[test]
fn test_factorial() {
    let source = include_str!("programs/control_flow/factorial.mth");
    let result = common::compile_and_execute(source);
    assert_eq!(result, 120);
}
