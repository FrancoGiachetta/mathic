mod common;

#[test]
fn test_basic_declaration() {
    let source = include_str!("programs/variables/basic_declaration.mth");
    let result = common::compile_and_execute(source);
    assert_eq!(result, 42);
}

#[test]
fn test_reassignment() {
    let source = include_str!("programs/variables/reassignment.mth");
    let result = common::compile_and_execute(source);
    assert_eq!(result, 20);
}

#[test]
fn test_multiple_variables() {
    let source = include_str!("programs/variables/multiple_variables.mth");
    let result = common::compile_and_execute(source);
    assert_eq!(result, 15);
}

#[test]
fn test_variable_increment() {
    let source = include_str!("programs/variables/variable_increment.mth");
    let result = common::compile_and_execute(source);
    assert_eq!(result, 20);
}
