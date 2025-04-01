#![allow(dead_code)]
#![allow(unused_imports)]
use crate::*;
use mockall::automock;
use serial_test::serial;
use std::collections::HashMap;
use std::panic::resume_unwind;
use sysinfo::System;

// Function -> ()
#[test]
fn trap_mem_fn_called_once_and_works() {
    // arrange
    let mock = MockTestEntity::test_method_context();
    mock.expect().times(1).return_const(());

    // act
    let result = trap_mem!(MockTestEntity::test_method());

    // assert
    assert_eq!((), result);
}

#[test]
fn trap_mem_named_method_works() {
    // arrange
    let mock = MockTestEntity::test_method_context();
    mock.expect().times(1).return_const(());

    // act
    let result = trap_mem!("test_method", MockTestEntity::test_method());

    // assert
    assert_eq!((), result);
}

// Function -> i32
#[test]
fn trap_mem_fn_called_once_and_returns_value() {
    // arrange
    let mock = MockTestEntity::test_method_with_value_context();
    mock.expect().times(1).return_const(3);

    // act
    let result = trap_mem!(MockTestEntity::test_method_with_value());

    // assert
    assert_eq!(3, result);
}

#[test]
fn trap_mem_named_method_returns_value() {
    // arrange
    let mock = MockTestEntity::test_method_with_value_context();
    mock.expect().times(1).return_const(3);

    // act
    let result = trap_mem!(
        "test_method_with_value",
        MockTestEntity::test_method_with_value()
    );

    // assert
    assert_eq!(3, result);
}

// Function -> HashMap
#[test]
#[serial]
fn trap_mem_fn_called_once_and_returns_hashmap() {
    // arrange
    let mock = MockTestEntity::test_method_creates_huge_map_context();
    mock.expect()
        .times(1)
        .return_const(TestEntity::test_method_creates_huge_map());

    // act
    let result = trap_mem!(MockTestEntity::test_method_creates_huge_map());

    // assert
    assert_eq!(1_000_000, result.len());
}

#[test]
#[serial]
fn trap_mem_named_method_returns_hashmap() {
    // arrange
    let expected = TestEntity::get_huge_map();
    let mock = MockTestEntity::test_method_creates_huge_map_context();
    mock.expect().times(1).return_const(expected.clone());

    // act
    let result = trap_mem!(
        "test_method_with_value",
        MockTestEntity::test_method_creates_huge_map()
    );

    // assert
    assert_eq!(expected.len(), result.len());
}

// Expression
#[test]
fn trap_mem_expr_works() {
    // act
    let result = trap_mem!({
        let _a = 1;
        let _b = 2;
        // do nothing
    });

    // assert
    assert_eq!((), result);
}

#[test]
fn trap_mem_expr_returns_value() {
    // act
    let result = trap_mem!({
        let _a = 1;
        let _b = 2;
        _a + _b
    });

    // assert
    assert_eq!(3, result);
}

// Units

#[test]
#[serial]
fn trap_mem_set_kb() {
    // arrange
    let expected = TestEntity::get_huge_map();
    let mock = MockTestEntity::test_method_creates_huge_map_context();
    mock.expect().times(1).return_const(expected.clone());

    // act
    let result = trap_mem!(
        "test_method_with_value",
        MemUnits::Kb,
        MockTestEntity::test_method_creates_huge_map()
    );

    // assert
    assert_eq!(expected.len(), result.len());
}

#[test]
#[serial]
fn trap_mem_set_mb() {
    // arrange
    let expected = TestEntity::get_huge_map();
    let mock = MockTestEntity::test_method_creates_huge_map_context();
    mock.expect().times(1).return_const(expected.clone());

    // act
    let result = trap_mem!(
        "test_method_with_value",
        MemUnits::Mb,
        MockTestEntity::test_method_creates_huge_map()
    );

    // assert
    assert_eq!(expected.len(), result.len());
}

struct TestEntity {}

#[automock]
impl TestEntity {
    fn test_method() {}

    fn test_method_with_value() -> i32 {
        1 + 2
    }

    fn test_method_creates_huge_map() -> HashMap<u64, u64> {
        Self::get_huge_map()
    }

    fn get_huge_map() -> HashMap<u64, u64> {
        let mut map = HashMap::with_capacity(1_000_000);
        for i in 0..1_000_000u64 {
            map.insert(i, i);
        }
        map
    }
}
