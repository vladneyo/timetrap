#![allow(dead_code)]
use mockall::automock;
#[test]
fn test_inner_exp_was_called(){
    let mock = MockTestEntity::test_method_context();

    mock.expect().times(1).return_const(());

    assert_eq!((), MockTestEntity::test_method())
}


struct TestEntity{}

#[automock]
impl TestEntity{
    fn test_method(){}
}