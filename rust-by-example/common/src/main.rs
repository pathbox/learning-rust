fn main() {
    println!("Hello, world!");
    // let x = 5; // 不可变变量声明
    let mut x = 5; // 可变变量声明
    x = 6; // 不能直接赋值，需要let先声明
    println!("The value of x is: {}", x);

    let _sum = 5 + 10;
    let _difference = 95.5 -9.1;
    let _product = 4 * 10;
    let _quotient = 56.7 /11.1;
    let _remainder = 43 % 2;  // rust 常亮建议用_开头

    let _t = true;
    let _f: bool = false; // 显示指定类型注解

    let _c = 'z';
    let _heart_eyed_cat = '😻';
    let _c = 'a'; // 重新初始赋值
    let _ss = "abcd";
    println!("The value of _ss is: {}", _ss);
    println!("The value of cat is: {}", _heart_eyed_cat);
}
