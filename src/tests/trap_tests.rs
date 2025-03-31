#![allow(dead_code)]
use crate::*;
use mockall::automock;

// Function -> ()
#[test]
fn trap_fn_called_once_and_works() {
    // arrange
    let mock = MockTestEntity::test_method_context();
    mock.expect().times(1).return_const(());

    // act
    let result = trap!(MockTestEntity::test_method());

    // assert
    assert_eq!((), result);
}

#[test]
fn trap_named_method_works() {
    // arrange
    let mock = MockTestEntity::test_method_context();
    mock.expect().times(1).return_const(());

    // act
    let result = trap!("test_method", MockTestEntity::test_method());

    // assert
    assert_eq!((), result);
}

// Function -> i32
#[test]
fn trap_fn_called_once_and_returns_value() {
    // arrange
    let mock = MockTestEntity::test_method_with_value_context();
    mock.expect().times(1).return_const(3);

    // act
    let result = trap!(MockTestEntity::test_method_with_value());

    // assert
    assert_eq!(3, result);
}

#[test]
fn trap_named_method_returns_value() {
    // arrange
    let mock = MockTestEntity::test_method_with_value_context();
    mock.expect().times(1).return_const(3);

    // act
    let result = trap!(
        "test_method_with_value",
        MockTestEntity::test_method_with_value()
    );

    // assert
    assert_eq!(3, result);
}

// Expression
#[test]
fn trap_expr_works() {
    // act
    let result = trap!({
        let _a = 1;
        let _b = 2;
        // do nothing
    });

    // assert
    assert_eq!((), result);
}

#[test]
fn trap_expr_returns_value() {
    // act
    let result = trap!({
        let _a = 1;
        let _b = 2;
        _a + _b
    });

    // assert
    assert_eq!(3, result);
}

struct TestEntity {}

#[automock]
impl TestEntity {
    fn test_method() {}

    fn test_method_with_value() -> i32 {
        1 + 2
    }
}
