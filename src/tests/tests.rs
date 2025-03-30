#![allow(dead_code)]
use mockall::automock;
use crate::trap;

#[test]
fn trap_expr_works(){
    // arrange
    let mock = MockTestEntity::test_method_context();

    // act
    mock.expect().times(1).return_const(());

    // assert
    assert_eq!((), trap!(MockTestEntity::test_method()));
}

#[test]
fn trap_name_expr_works(){
    // arrange
    let mock = MockTestEntity::test_method_context();

    // act
    mock.expect().times(1).return_const(());

    // assert
    assert_eq!((), trap!("test_method", MockTestEntity::test_method()));
}


struct TestEntity{}

#[automock]
impl TestEntity{
    fn test_method(){}
}