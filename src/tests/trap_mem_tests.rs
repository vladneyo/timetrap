use crate::*;
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};

fn test_method(counter: &AtomicUsize) {
    counter.fetch_add(1, Ordering::SeqCst);
}

fn test_method_with_value(counter: &AtomicUsize) -> i32 {
    counter.fetch_add(1, Ordering::SeqCst);
    1 + 2
}

fn test_method_creates_huge_map(counter: &AtomicUsize) -> HashMap<u64, u64> {
    counter.fetch_add(1, Ordering::SeqCst);

    let mut map = HashMap::with_capacity(100_000);
    for i in 0..100_000u64 {
        map.insert(i, i);
    }
    map
}

// Function -> ()
#[test]
fn trap_mem_fn_called_once_and_works() {
    // arrange
    let call_counter = AtomicUsize::new(0);

    // act
    trap_mem!(test_method(&call_counter));

    // assert
    assert_eq!(1, call_counter.load(Ordering::SeqCst));
}

#[test]
fn trap_mem_named_method_works() {
    // arrange
    let call_counter = AtomicUsize::new(0);

    // act
    trap_mem!("test_method", test_method(&call_counter));

    // assert
    assert_eq!(1, call_counter.load(Ordering::SeqCst));
}

// Function -> i32
#[test]
fn trap_mem_fn_called_once_and_returns_value() {
    // arrange
    let expected = 3;
    let call_counter = AtomicUsize::new(0);

    // act
    let result = trap_mem!(test_method_with_value(&call_counter));

    // assert
    assert_eq!(expected, result);
    assert_eq!(1, call_counter.load(Ordering::SeqCst));
}

#[test]
fn trap_mem_named_method_returns_value() {
    // arrange
    let expected = 3;
    let call_counter = AtomicUsize::new(0);

    // act
    let result = trap_mem!(
        "test_method_with_value",
        test_method_with_value(&call_counter)
    );

    // assert
    assert_eq!(expected, result);
    assert_eq!(1, call_counter.load(Ordering::SeqCst));
}

#[test]
fn trap_mem_named_method_with_color_returns_value() {
    // arrange
    let expected = 3;
    let call_counter = AtomicUsize::new(0);

    // act
    let result = trap_mem!(
        "test_method_with_value",
        color = Colors::Yellow,
        test_method_with_value(&call_counter)
    );

    // assert
    assert_eq!(expected, result);
    assert_eq!(1, call_counter.load(Ordering::SeqCst));
}

// Function -> HashMap
#[test]
fn trap_mem_fn_called_once_and_returns_hashmap() {
    // arrange
    let call_counter = AtomicUsize::new(0);
    let expected_len = 100_000;

    // act
    let result = trap_mem!(test_method_creates_huge_map(&call_counter));

    // assert
    assert_eq!(expected_len, result.len());
    assert_eq!(1, call_counter.load(Ordering::SeqCst));
}

#[test]
fn trap_mem_named_method_returns_hashmap() {
    // arrange
    let call_counter = AtomicUsize::new(0);
    let expected_len = 100_000;

    // act
    let result = trap_mem!(
        "test_method_with_value",
        test_method_creates_huge_map(&call_counter)
    );

    // assert
    assert_eq!(expected_len, result.len());
    assert_eq!(1, call_counter.load(Ordering::SeqCst));
}

// Expression
#[test]
fn trap_mem_expr_works() {
    // arrange
    let mut was_called = false;

    // act
    trap_mem!({
        was_called = true;
    });

    // assert
    assert!(was_called);
}

#[test]
fn trap_mem_expr_returns_value() {
    // arrange
    let expected = 3;

    // act
    let result = trap_mem!({
        let _a = 1;
        let _b = 2;
        _a + _b
    });

    // assert
    assert_eq!(expected, result);
}

// Units

#[test]
fn trap_mem_set_kb() {
    // arrange
    let call_counter = AtomicUsize::new(0);
    let expected_len = 100_000;

    // act
    let result = trap_mem!(
        "test_method_with_value",
        MemUnits::Kb,
        test_method_creates_huge_map(&call_counter)
    );

    // assert
    assert_eq!(expected_len, result.len());
    assert_eq!(1, call_counter.load(Ordering::SeqCst));
}

#[test]
fn trap_mem_set_mb() {
    // arrange
    let call_counter = AtomicUsize::new(0);
    let expected_len = 100_000;

    // act
    let result = trap_mem!(
        "test_method_with_value",
        MemUnits::Mb,
        test_method_creates_huge_map(&call_counter)
    );

    // assert
    assert_eq!(expected_len, result.len());
    assert_eq!(1, call_counter.load(Ordering::SeqCst));
}

#[test]
fn trap_mem_set_mb_with_color() {
    // arrange
    let call_counter = AtomicUsize::new(0);
    let expected_len = 100_000;

    // act
    let result = trap_mem!(
        "test_method_with_value",
        MemUnits::Mb,
        color = Colors::Magenta,
        test_method_creates_huge_map(&call_counter)
    );

    // assert
    assert_eq!(expected_len, result.len());
    assert_eq!(1, call_counter.load(Ordering::SeqCst));
}
