use std::fmt;

#[derive(Debug)]
struct MinMax(i64, i64);

// Implement `Display` for `MinMax`
impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // use self.number to refer to each positional data point
        write!(f, "({},{})", self.0, self.1)
    }
}

#[derive(Debug)]
struct Point2D{
    x: f64,
    y: f64,
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x===: {}, y===: {}", self.x, self.y)
    }
}

#[derive(Debug)]
struct Deep(Structure);

#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

fn main() {
    println!("{0},this is {1}. {1}, this is {0}", "Alice", "Bob");
    // Special formatting can be specified after a `:`.
    println!("{} of {:b} people know binary, the other half doesn't", 1, 2);
    // You can right-align text with a specified width. This will output
    // "     1". 5 white spaces and a "1".
    println!("{number:>width$}",number=1, width=6);
    // Rust even checks to make sure the correct number of arguments are
    // used.
    println!("My name is {0}, {0}", "Bond");

    println!("{1:?} {0:?} is the {actor:?} name.",
             "Slater",
             "Christian",
             actor="actor's");
     println!("Now {:?} will print!", Structure(3));
     println!("Now {:?} will print!", Deep(Structure(7)));

    let name = "Peter";
    let age = 27;
    let peter = Person {name, age};

    println!("{:#?}", peter);

    let minmax = MinMax(0, 14);

    println!("Compare structures:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!("The big range is {big} and the small is {small}", small=small_range, big = big_range);

    let point = Point2D {x: 3.3, y: 7.2};

    println!("Compare points:");
    println!("Display: {}", point); // 如果没有impl fmt::Display for Point2D 这里是会报错的，{} 是默认的formatter 不能格式化打印struct
    println!("Debug: {:?}", point);
}
