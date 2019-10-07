//Rust 实现了泛型，使得使用泛型类型参数的代码相比使用具体类型并没有任何速度上的损失。

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];



    let integer = Point{x: 5, y: 10};
    let float = Point{x: 1.0, y: 3.0};

    println!("p.x = {}", integer.x());

    let p1 = Point2 { x: 5, y: 10.4 };
    let p2 = Point2 { x: "Hello", y: 'c'};

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

//fn largest<T>(list: &[T]) -> T {
//    let mut largest = list[0];
//
//    for &item in list.iter() {
//        if item > largest {
//            largest = item;
//        }
//    }
//    largest
//}

// x y 是相同的类型
struct Point<T> {
    x: T,
    y: T,
}

struct Point2<T,U> {
    x: T,
    y: U,
}

impl<T, U> Point2<T, U> {
    fn mixup<V,W>(self, other: Point2<V,W>) -> Point2<T, W> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

