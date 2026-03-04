mod common;

#[test]
fn test_simple_function() {
    let source = include_str!("programs/functions/simple_function.mth");
    let result = common::compile_and_execute(source);
    assert_eq!(result, 7);
}

#[test]
fn test_fibonacci() {
    let source = include_str!("programs/functions/fibonacci.mth");
    let result = common::compile_and_execute(source);
    assert_eq!(result, 55);
}

#[test]
fn test_factorial() {
    let source = include_str!("programs/functions/factorial.mth");
    let result = common::compile_and_execute(source);
    assert_eq!(result, 120);
}

#[test]
fn test_nested_calls() {
    let source = include_str!("programs/functions/nested_calls.mth");
    let result = common::compile_and_execute(source);
    assert_eq!(result, 26);
}
