pub fn largest<T: std::cmp::PartialOrd + Copy>(list: &[T]) -> T {
    // 默认为move语义，不能通过引用把所有权拿走，除非你Copy一份，原来那一份你还是不能动。
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

pub fn add<T: std::ops::Add<Output=T>>(a: T, b: T) -> T {
    a + b
}


struct Point<T> {
    x: T,
    y: T,
}

// 需要使用impl<T>
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

pub fn get_point_x() {
    let p = Point {
        x: 1.2,
        y: 1.3,
    };
    println!("p.x: {}", p.x());
}

// 通过引用，我们可以很轻松的解决处理任何类型数组的问题
pub fn display_array<T: std::fmt::Debug>(arr: &[T]) {
    println!("{:?}", arr);
}

// const 泛型，也就是针对值的泛型
pub fn display_array2<T: std::fmt::Debug, const N: usize>(arr: [T; N]) {
    println!("{:?}", arr);
}