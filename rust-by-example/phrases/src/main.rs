extern crate phrases;

use phrases::english::{greetings,farewells};

// use phrases::english::greetings::hello;
// use phrases::english::farewells::goodbye;

fn main() {
  println!("Hello in English: {}",  greetings::hello()); 
  println!("Goodbye in English: {}",  farewells::goodbye());
  println!("Hello in Japanese: {}", phrases::japanese::greetings::hello());
  println!("Goodbye in Japanese: {}", phrases::japanese::farewells::goodbye());
}

// extern crate phrases as sayings;
// use sayings::japanese::greetings as ja_greetings;
// use sayings::japanese::farewells::*;
// use sayings::english::{self, greetings as en_greetings, farewells as en_farewells};