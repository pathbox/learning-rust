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

    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let string1 = String::from("long string is long");
    let string2 = String::from("xyz");
    let result;
    {
//        let string2 = String::from("xyz"); // `string2` does not live long enough
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
}

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

pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
   if x.len() > y.len() {
         x
   } else {
      y
  }
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}


