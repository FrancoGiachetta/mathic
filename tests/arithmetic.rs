mod common;

#[test]
fn test_addition() {
    let source = include_str!("programs/arithmetic/addition.mth");
    let result = common::compile_and_execute(source);
    assert_eq!(result, 8);
}

#[test]
fn test_subtraction() {
    let source = include_str!("programs/arithmetic/subtraction.mth");
    let result = common::compile_and_execute(source);
    assert_eq!(result, 6);
}

#[test]
fn test_multiplication() {
    let source = include_str!("programs/arithmetic/multiplication.mth");
    let result = common::compile_and_execute(source);
    assert_eq!(result, 42);
}

#[test]
fn test_division() {
    let source = include_str!("programs/arithmetic/division.mth");
    let result = common::compile_and_execute(source);
    assert_eq!(result, 5);
}

#[test]
fn test_order_of_operations() {
    let source = include_str!("programs/arithmetic/order_of_operations.mth");
    let result = common::compile_and_execute(source);
    assert_eq!(result, 14);
}
