use crate::*;
use std::sync::atomic::{AtomicUsize, Ordering};

fn test_method(counter: &AtomicUsize) {
    counter.fetch_add(1, Ordering::SeqCst);
}

fn test_method_with_value(counter: &AtomicUsize) -> i32 {
    counter.fetch_add(1, Ordering::SeqCst);
    1 + 2
}

// Function -> ()
#[test]
fn trap_fn_called_once_and_works() {
    // arrange
    let call_counter = AtomicUsize::new(0);

    // act
    trap!(test_method(&call_counter));

    // assert
    assert_eq!(1, call_counter.load(Ordering::SeqCst));
}

#[test]
fn trap_named_method_works() {
    // arrange
    let call_counter = AtomicUsize::new(0);

    // act
    trap!("test_method", test_method(&call_counter));

    // assert
    assert_eq!(1, call_counter.load(Ordering::SeqCst));
}

// Function -> i32
#[test]
fn trap_fn_called_once_and_returns_value() {
    // arrange
    let expected = 3;
    let call_counter = AtomicUsize::new(0);

    // act
    let result = trap!(test_method_with_value(&call_counter));

    // assert
    assert_eq!(expected, result);
    assert_eq!(1, call_counter.load(Ordering::SeqCst));
}

#[test]
fn trap_named_method_returns_value() {
    // arrange
    let expected = 3;
    let call_counter = AtomicUsize::new(0);

    // act
    let result = trap!(
        "test_method_with_value",
        test_method_with_value(&call_counter)
    );

    // assert
    assert_eq!(expected, result);
    assert_eq!(1, call_counter.load(Ordering::SeqCst));
}

#[test]
fn trap_named_method_with_color_returns_value() {
    // arrange
    let expected = 3;
    let call_counter = AtomicUsize::new(0);

    // act
    let result = trap!(
        "test_method_with_value",
        color = Colors::Green,
        test_method_with_value(&call_counter)
    );

    // assert
    assert_eq!(expected, result);
    assert_eq!(1, call_counter.load(Ordering::SeqCst));
}

#[test]
fn trap_unnamed_expr_with_color_returns_value() {
    // arrange
    let expected = 3;

    // act
    let result = trap!(color = Colors::Cyan, {
        let a = 1;
        let b = 2;
        a + b
    });

    // assert
    assert_eq!(expected, result);
}

// Expression
#[test]
fn trap_expr_works() {
    // arrange
    let mut was_called = false;

    // act
    trap!({
        was_called = true;
    });

    // assert
    assert!(was_called);
}

#[test]
fn trap_expr_returns_value() {
    // arrange
    let expected = 3;

    // act
    let result = trap!({
        let _a = 1;
        let _b = 2;
        _a + _b
    });

    // assert
    assert_eq!(expected, result);
}
