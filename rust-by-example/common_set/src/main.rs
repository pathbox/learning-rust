enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}


fn main() {
    println!("Common Set");

    let v:Vec<i32> = Vec::new();

    let v = vec![1,2,3];

    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    // match v.get(2) {
    //     Some(third) => println!("The third element is {}", third),
    //     None => println!("There is no third element."),
    // }
    
    let v = vec![100, 200, 300];
    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![100,200,300];
    for i in &mut v {
        *i += 50;
        println!("{}", i);
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];


    let data1 = "initial contents";

    let s = data1.to_string();
    let s1 = String::from("initial contents");

    let mut s2 = String::from("foo");
    let s3 = " bar";
    s2.push_str(s3);
    println!("s2 is {}", s2);

    let s4 = String::from("Hello, ");
    let s5 = String::from("world!");
    let s6 = s4 + &s5; // 注意 s4 被移动了，不能继续使用
    println!("s6 is {}", s6);

    let s7 = String::from("tic");
    let s8 = String::from("tac");
    let s9 = String::from("toe");

    let s10 = format!("{}-{}-{}", s7, s8, s9); // 格式化字符串输出
    println!("s10 is {}", s10);
}
