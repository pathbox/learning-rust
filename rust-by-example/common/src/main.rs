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

    // 元组类型
    // 元组是一个将多个其他类型的值组合进一个复合类型的主要方式。
    // 我们使用包含在圆括号中的逗号分隔的值列表来创建一个元组。元组中的每一个位置都有一个类型，而且这些不同值的类型也不必是相同的
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x,y,z) = tup;
    println!("The tup is: {:?}", tup);
    println!("The value of y is: {}", y);
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
    let months = ["January", "February", "March", "April", "May", "June", "July","August", "September", "October", "November", "December"];

    println!("The value of Januray is: {}", months[0]);

    another_function(5, 6);

    let x = five();
    println!("The value of x is: {}", x);

    let x = plus_one(5);

    println!("The value of x is: {}", x);


    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    assert_eq!(result, 20);

    for number in (1..4).rev() { // 反序输出 只会到3
         println!("{} !", number);
    }
     
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn ifelseresult() {
    let condition = true; 

    let str = if condition {
        "yes"
    }else {
        "no"
    };
}

fn while_fn() {
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);

        number = number -1;
    }
    println!("LIFTOFF!!!");

    let a = [1,2,3,4,5];

    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index = index + 1;
    }

    for element in a.iter() {
       println!("the value is: {}", element);
    }

    for number in (1..4).rev() { // 反序输出
         println!("{}!", number);
    }
}

