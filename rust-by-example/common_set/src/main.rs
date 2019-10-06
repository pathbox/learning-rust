fn main() {
    println!("Common Set");

    let v:Vec<i32> = Vec::new();

    let v = vec![1,2,3];

    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    // match v.get(2) {
    //     Some(third) => println!("The third element is {}", third),
    //     None => println!("There is no third element."),
    // }
}
