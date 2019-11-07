enum List {
    Cons(i32, Box<List>),
    Nil,
}


use List::{Cons, Nil};


struct CustomSmartPointer {
    data: String,
}

// 指定在值离开作用域时应该执行的代码的方式是实现 Drop trait。Drop trait 要求实现一个叫做 drop 的方法，它获取一个 self 的可变引用。为了能够看出 Rust 何时调用 drop，让我们暂时使用 println! 语句实现 drop
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let b = Box::new(5);
    println!("b = {}", b);

    let list = Cons(1,
        Box::new(Cons(2,
            Box::new(Cons(3,
                Box::new(Nil)
            ))
        ))
    );

    let c = CustomSmartPointer {data: String::from("my stuff")};
    let d = CustomSmartPointer {data: String::from("other stuff")};
    println!("CustomSmartPointers created.");

}
