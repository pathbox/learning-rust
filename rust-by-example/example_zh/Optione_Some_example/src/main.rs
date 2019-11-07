fn main() {
    println!("Hello, world!");

    // 1. some=>不同的结构
    let v = vec![1,2];
    let res: Option<Vec<_>> = v.iter().map(|x| Some(x * 2)).collect();
    // assert_eq!(res, Some([2,4]));

    // 2、some单个值可以map
    let mut number = Some(10);
    let var = number.map(|n| 2 * n);
    println!("var: {:?}", var);

    // 3、Some,None
    let x = Some("string");
    let v: Vec<&str> = x.into_iter().collect();
    assert_eq!(v, ["string"]);

    let x = None;
    let v: Vec<&str> = x.into_iter().collect();
    assert!(v.is_empty());
    // 4、clone: Option<&T> => Option
    let x = 12;
    let opt_x = Some(&x);
    assert_eq!(opt_x, Some(&12));
    let cloned = opt_x.cloned();
    assert_eq!(cloned, Some(12));

    // 5、take : 淘空 
    let mut x = Some(2);
    x.take();
    assert_eq!(x, None);

    let mut x: Option<u32> = None;
    x.take();
    assert_eq!(x, None);
}


