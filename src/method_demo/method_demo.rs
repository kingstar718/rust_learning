pub struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

impl Circle {
    // 这种定义在 impl 中且没有 self 的函数被称之为关联函数：
    // 因为它没有 self，不能用 f.read() 的形式调用，因此它是一个函数而不是方法，
    // 它又在 impl 中，与结构体紧密关联，因此称为关联函数。
    // new是Circle的关联函数，因为它的第一个参数不是self，且new并不是关键字
    // 这种方法往往用于初始化当前结构体的实例
    // Rust 中有一个约定俗成的规则，使用 new 来作为构造器的名称
    fn new(x: f64, y: f64, radius: f64) -> Circle {
        Circle {
            x,
            y,
            radius,
        }
    }

    // Circle的方法，&self表示借用当前的Circle结构体
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }

    // self 表示 Rectangle 的所有权转移到该方法中，这种形式用的较少
    // &self 表示该方法对 Rectangle 的不可变借用
    // &mut self 表示可变借用

    // 一般来说，方法和字段同名，用于实现getter
    pub fn radius(&self) -> f64 {
        return self.radius;
    }

    // Rust 并没有一个与 -> 等效的运算符；
    // 相反，Rust 有一个叫 自动引用和解引用的功能。
    // 方法调用是 Rust 中少数几个拥有这种行为的地方。
    //
    // 他是这样工作的：当使用 object.something() 调用方法时，Rust 会自动为 object 添加 &、&mut 或 * 以便使 object 与方法签名匹配。
    // 也就是说，这些代码是等价的：
    // p1.distance(&p2);
    // (&p1).distance(&p2);
    // 第一行看起来简洁的多。这种自动引用的行为之所以有效，是因为方法有一个明确的接收者———— self 的类型。
    // 在给出接收者和方法名的前提下，Rust 可以明确地计算出方法是仅仅读取（&self），做出修改（&mut self）或者是获取所有权（self）。
    // 事实上，Rust 对方法接收者的隐式借用让所有权在实践中更友好。
}

// 多个impl也可以
impl Circle {
    fn can_hold(&self, other: &Circle) -> bool {
        self.x > other.x && self.y > other.y
    }
}

pub fn get_radius() {
    let circle = Circle::new(1.1, 1.2, 1.3);
    println!("{}", circle.radius());
}

// 为枚举定义方法
#[allow(unused)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // 在这里定义方法体
    }
}