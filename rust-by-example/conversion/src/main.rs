use std::convert::From; 
use std::convert::TryFrom;
use std::convert::TryInto;

#[derive(Debug)]
struct Number {
    value: i32,
}

#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(())
        }
    }
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

    let result: Result<EvenNumber, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    let result: Result<EvenNumber, ()> = 5i32.try_into();
    assert_eq!(result, Err(()));

    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "100".parse::<i32>().unwrap();

    let sum = parsed + turbo_parsed;
    println!("Sum: {:?}", sum);

    let mut count = 0u32;

    loop {
        count += 1;

        if count == 3 {
            println!("three");
            continue;
        }

        println!("{}", count);

        if count == 10 {
            println!("OK, it's enough");

            break;
        }
    }

    'outer: loop {
        println!("Entered the outer loop");

        'inner: loop {
            println!("Entered the inner loop");

            // This would break only the inner loop
            //break;

            // This breaks the outer loop
            break 'outer;
        }

        println!("This point will never be reached");
    }

    println!("Exited the outer loop");

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    assert_eq!(result, 20);

    let names = vec!["Bob", "Frank", "Feeris"];
    for name in names.iter() {
        &"Ferris" => println!("There is a rustacean among us!"),
        _ => println!("Hello {}", name);
    }
    for name in names.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
}
