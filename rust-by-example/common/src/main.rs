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

    // let s1 = String::from("hello");
    // let s2 = s1; // 这是移动 而不是浅拷贝 所以s1已结被释放了
    // println!("{}, world!", s1);// 报错s1 value borrowed here after move

    // clone 克隆
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    let s3 = "nice";
    let s4 = s3;  // 这是浅拷贝
    println!("{}",s4); // 这样却是可以正确的



    let s = String::from("hello");  // s 进入作用域

    takes_ownership(s);             // s 的值移动到函数里 ...
                                    // ... 所以到这里不再有效

    let x = 5;                      // x 进入作用域

    makes_copy(x);                  // x 应该移动函数里，
                                    // 但 i32 是 Copy 的，所以在后面可继续使用 x


    let s1 = gives_ownership();         // gives_ownership 将返回值
                                        // 移给 s1

    let s2 = String::from("hello");     // s2 进入作用域

    let s3 = takes_and_gives_back(s2);  // s2 被移动到
                                        // takes_and_gives_back 中, 
                                        // 它也将返回值移给 s3
     
}

fn gives_ownership() -> String {             // gives_ownership 将返回值移动给
                                             // 调用它的函数

    let some_string = String::from("hello"); // some_string 进入作用域.

    some_string                              // 返回 some_string 并移出给调用的函数
}

// takes_and_gives_back 将传入字符串并返回该值
fn takes_and_gives_back(a_string: String) -> String { // a_string 进入作用域

    a_string  // 返回 a_string 并移出给调用的函数
}

fn takes_ownership(some_string: String) { // some_string 进入作用域
    println!("{}", some_string);
} // 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放

fn makes_copy(some_integer: i32) { // some_integer 进入作用域
    println!("{}", some_integer);
} // 这里，some_integer 移出作用域。不会有特殊操作

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

