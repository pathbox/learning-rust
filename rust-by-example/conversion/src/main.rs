use std::convert::From; 

#[derive(Debug)]
struct Number {
    value: i32,
}

// impl From method
impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

fn main() {
    let num = Number::from(30);
    println!("My number is {:?}", num);
    println!("My number is {}", num.value);

    let int = 5;
    let num: Number = int.into(); // The same as let num = Number::from(5);
    println!("My number is {:?}", num);
}
