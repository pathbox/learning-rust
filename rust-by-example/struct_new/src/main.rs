impl Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
}
struct CircleBuilder {
    x: f64,
    y: f64,
    radius: f64,
}

impl CircleBuilder {
    fn new() -> CircleBuilder {
        CircleBuilder { x: 0.0, y: 0.0, radius: 1.0, }
    }
    fn x(&mut self, coordinate: f64) -> &mut CircleBuilder {
        self.x = coordinate;
        self
    }
    fn y(&mut self, coordinate: f64) -> &mut CircleBuilder {
        self.y = coordinate;
        self
    }
    fn radius(&mut self, radius: f64) -> &mut CircleBuilder {
        self.radius = radius;
        self
    }
    fn finalize(&self) -> Circle {
        Circle { x: self.x, y: self.y, radius: self.radius }
    }
}

trait HasArea {
    fn area(&self) -> f64; 
    fn is_larger(&self, &Self) -> bool;
}

impl HasArea for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }

    fn is_larger(&self, other: &self) -> bool {
        self.area() > other.area()
    }
}

use std::fmt::Debug;

fn bar<T, K>(x: T, y: K) where T: Clone, K: Clone + Debug {
    x.clone();
    y.clone();
    println!("{:?}",y);
}

fn main() {
    let c = CircleBuilder::new()
                .x(1.0)
                .y(2.0)
                .radius(2.0)
                .finalize();
    
    println!("area: {}", c.area());
    println!("x: {}", c.x);
    println!("y: {}", c.y);       
    bar("Hello", "world");   
}

trait ConvertTo<Output> {
    fn convert(&self) -> Output;
}
impl ConvertTo<i64> for i32 {
    fn convert(&self) -> i64 { *self as i64 }
}
// Can be called with T == i32.
fn normal<T: ConvertTo<i64>>(x: &T) -> i64 {
    x.convert()
}
// Can be called with T == i64.
fn inverse<T>(x: i32) -> T
    // This is using ConvertTo as if it were "ConvertTo<i64>
    ".
    where i32: ConvertTo<T> {
        x.convert()
}

//这突显出了 where 从句的额外的功能：它允许限制的左侧可以是任意类型（在这
// 里是 i32 ），而不仅仅是一个类型参数（比如 T ）。在这个例子中， i32 必须
// 实现 ConvertTo<T> 。不同于定义 i32 是什么（因为这是很明显的），这里
// 的 where 从句限制了 T