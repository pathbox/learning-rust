fn main() {

    let mut user = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        active: true,
        sign_in_count: 1,
    };

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("email: {}", user.email);

    let rect1 = (30,50);

     println!(
        "The area of the rectangle is {} square pixels.",
        area(rect1)
    );

    let rect2 = Rectangle {width: 30, height: 50}; 
     println!(
        "The area2 of the rectangle is {} square pixels.",
        area2(&rect2)
    );

    let rect3 = Rectangle { width: 30, height: 50 };

    println!("rect3 is {:?}", rect3);
    println!("rect3 is {:#?}", rect3);

    let mut rect4 = Rectangle {width: 30, height: 50};

    let name = String::from("Cary");
    println!(
        "The area of the rectangle is {} square pixels.",
        rect4.area(name)
    );

}

struct User {
    username: String,
    email: String, 
    sign_in_count: u64,
    active: bool,
}


fn build_user(email: String, username: String) -> User {
    User {
        email: email, 
        username: username, 
        active: true, 
        sign_in_count: 1,
    }
}

fn area(dimensions: (u32, u32)) ->u32 {
    dimensions.0 * dimensions.1 
}

#[derive(Debug)] //增加注解来派生 Debug trait，并使用调试格式打印 Rectangle 实例
struct Rectangle {
    width: u32, 
    height: u32, 
}

// 给struct定义方法

impl Rectangle {
    // add code here
    fn area(&self, name: String) -> u32 {
        println!("The name: {}", name);
        self.width * self.height 
    }
}

fn area2(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

// # #[derive(Debug,Copy,Clone)]
// # struct Point {
// #     x: f64,
// #     y: f64,
// # }
// #
// # impl Point {
// #    fn distance(&self, other: &Point) -> f64 {
// #        let x_squared = f64::powi(other.x - self.x, 2);
// #        let y_squared = f64::powi(other.y - self.y, 2);
// #
// #        f64::sqrt(x_squared + y_squared)
// #    }
// # }
// # let p1 = Point { x: 0.0, y: 0.0 };
// # let p2 = Point { x: 5.0, y: 6.5 };
// p1.distance(&p2);
// (&p1).distance(&p2);