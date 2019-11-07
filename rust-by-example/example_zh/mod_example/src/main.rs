use crate::sound1::instrument;
use std::collections::HashMap;
use std::io::Result as IoResult; // 重命名
use std::{cmp::Ordering, io}; // use std::cmp::Ordering; use std::io;
mod sound;

fn main() {
    let mut v = plant::Vegetable::new("squash", 10);

    v.name = String::from("butternut squash");
    println!("{} are delicious", v.name);
    // 如果将如下行取消注释代码将无法编译:
    // println!("The ID is {}", v.id);

    let order1 = menu1::Appetizer::Soup;

    instrument::clarinet();

    let mut map = HashMap::new();
    map.insert(1,2);
    performance_group1::clarinet_trio();
    performance_group1::instrument::clarinet(); // 重导出后能够使用到

    // 绝对路径
    crate::sound::instrument::clarinet();

    // 相对路径
    sound::instrument::clarinet();
}

mod performance_group1 {
    pub use crate::sound1::instrument;

    pub fn clarinet_trio() {
        instrument::clarinet();
        instrument::clarinet();
        instrument::clarinet();
    }
}

mod instrument1 {
    fn clarinet() {
        super::breathe_in();
    }
}

fn breathe_in() {
    // 函数体
}

mod plant {
    pub struct Vegetable {
        pub name: String, 
        id:i32,
    }

    impl Vegetable {
        pub fn new(name: &str, id: i32) -> Vegetable {
            Vegetable {
                name: String::from(name),
                id: id,
            }
        }
    }
}

mod menu1 {
    pub enum Appetizer {
        Soup, 
        Salad, 
    }
}

mod sound1 {
    pub mod instrument {
        pub fn clarinet() {
            // 函数体
            println!("Sound...");
        }
    }
}

mod performance_group {
    use crate::sound1::instrument;

    pub fn clarinet_trio() {
        instrument::clarinet();
        instrument::clarinet();
        instrument::clarinet();
    }
}