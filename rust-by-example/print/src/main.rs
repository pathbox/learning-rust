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
}
