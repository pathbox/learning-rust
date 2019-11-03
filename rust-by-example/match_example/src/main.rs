enum Color {
   Rgb(i32, i32, i32),
   Hsv(i32, i32, i32)
}

enum Message {
    Quit, 
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

struct Point {
    x: i32, 
    y: i32,
}

struct Point1 {
    x: i32,
    y: i32,
    z: i32,
}

fn main() {
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!(
                "Change the color to red {}, green {}, and blue {}",
                r,
                g,
                b
            )  
        },
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!(
                "Change the color to hue {}, saturation {}, and value {}",
                h,
                s,
                v
            )
        }
         _ => ()
    }

    let points = vec![
        Point { x: 0, y: 0 },
        Point { x: 1, y: 5 },
        Point { x:10, y: -3},
    ];

    let sum_of_squares: i32 = points.iter().map(|&Point {x, y }| x * x + y * y).sum();

    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {}, {}, {}", first, third, fifth)
        },
    }

    

    let origin = Point1 { x: 0, y: 0, z: 0 };

    match origin {
        Point1 { x, .. } => println!("x is {}", x),
    }
}
