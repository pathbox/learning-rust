struct Foo<'a> {
    x: &'a i32,
}

impl<'a> Foo<'a> {
    fn x(&self) -> &'a i32 {self.x}
}

fn main() {
    // let r; 
    // {
    //     let i = 1;
    //     r = &i; // Store reference of `i` in `r`.
    // } // `i` goes out of scope and is dropped.

    // println!("{}", r); // `r` still refers to `i`.  

    
    let line = "lang:en=Hello World!";
    let lang = "en";
    let v;
    {
        let p = format!("lang:{}=", lang); // -+ `p` comes into scope.
        v = skip_prefix(line, p.as_str()); // |
    } // -+ `p` goes out of scope.
    println!("{}", v);

    let y = &5;

    let f = Foo{ x: y };

    println!("x is: {}", f.x());
}
// fn skip_prefix(line: &str, prefix: &str) -> &str {
// // ...

// }

fn skip_prefix<'a,'b>(line: &'a str, prefix: &'b str) -> &'a str {
    // ...
    // 返回字符串和line一样 拥有相同的生命周期 从而让编译器在编译时明白而不报错
    return line;
}
// i 被释放了 最后println！ 的时候 r指向的是悬垂指针

//这里我们有个函数 skip_prefix ，它获取两个 &str 引用作为参数并返回一
//个 &str 引用。通过 line 和 p 的引用调用它：两个有不同生命周期的变量。现
//在 println! 那行代码的安全依赖于 skip_prefix 函数返回的引用是仍然存在
//的 line 还是已经释放掉的 p 。
//因为存在上述的歧义，Rust 将会拒绝编译示例代码。为了继续我们需要向编译器提
//供更多关于引用生命周期的信息。这可以通过再函数签名中显式标明生命周期来完

//让我们看看所做的修改，但是现在并不深入到语法--之后我们会讲到。第一个修改
//是再方法名后面加入了 <'a, 'b> 。这引入了两个生命周期参数 'a 和 'b 。接
//下来函数签名中的每个引用都关联了一个生命周期参数，通过再 & 之后加上生命
//周期的名字。这告诉了编译器不同引用的生命周期是如何关联的。
//这样编译器就能推断出 skip_prefix 函数的返回值与 line 参数有着相同的生命
//周期，这样就使得之前例子中 v 引用即使在 p 离开作用域之后也能安全使用。
//另外编译器能够检查 skip_prefix 返回值的用途，它也能确保之后的实现也遵守
//函数声明建立的约束。这在实现之后会介绍到的 trait 时显得尤为实用