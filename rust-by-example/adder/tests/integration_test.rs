use adder;

mod common;

#[test]
fn it_adds_two() {
    println!("==Here...==");
    common::setup();
    assert_eq!(4, adder::add_two(2));
}

//cargo test --test integration_test 指定该单元测试 只执行这一个单元测试