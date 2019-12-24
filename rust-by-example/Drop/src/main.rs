struct Firework {
    strength: i32,
}
impl Drop for Firework { // Drop 和 Firework 绑定
    fn drop(&mut self) {
        println!("BOOM times {}!!!", self.strength);
    }
}
    
fn main() {
    let firecracker = Firework { strength: 1 };
    let tnt = Firework { strength: 100 };
}

// 就是这样！ Drop 的机制非常简单，不过这有一些细节。例如，值会以与它们声明
// 相反的顺序被丢弃（dropped）,栈的方式丢弃。这是另一个例子：