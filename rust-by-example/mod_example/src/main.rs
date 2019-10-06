use crate::sound1::instrument;

fn main() {
    let mut v = plant::Vegetable::new("squash", 10);

    v.name = String::from("butternut squash");
    println!("{} are delicious", v.name);
    // 如果将如下行取消注释代码将无法编译:
    // println!("The ID is {}", v.id);

    let order1 = menu1::Appetizer::Soup;

    instrument::clarinet();
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