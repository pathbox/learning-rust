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

    // User {
    //     email,
    //     username,
    //     active: true,
    //     sign_in_count: 1,
    // }
}