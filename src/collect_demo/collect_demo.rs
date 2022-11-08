// 创建动态数组
pub fn create_vec() {
    let v: Vec<i32> = Vec::new();

    let mut v1 = Vec::new();
    // 通过push推导类型
    v1.push(1);

    let v3 = vec![1, 2, 3];

    println!("{:?}", v);
    println!("{:?}", v1);
    println!("{:?}", v3);
}

// 读取元素
pub fn read_vec() {
    let v = vec![1, 2, 3, 4, 5];
    // &v[2] 表示借用 v 中的第三个元素，最终会获得该元素的引用
    let third: &i32 = &v[2];
    println!("第三个元素 {}", third);

    // v.get(2) 也是访问第三个元素，但是有所不同的是，它返回了 Option<&T>，因此还需要额外的 match 来匹配解构出具体的值
    match v.get(2) {
        Some(third) => println!("第三个元素 {}", third),
        None => println!("没有第三个元素"),
    }

    // &v[100] 的访问方式会导致程序无情报错退出，因为发生了数组越界访问。
    // v.get 就不会，它在内部做了处理，有值的时候返回 Some(T)，无值的时候返回 None，因此 v.get 的使用方式非常安全。
}

pub fn batch_use() {
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];

    // 如果 first 在 v.push 之后不再使用，那么该段代码可以成功编译
    v.push(6);

    // 原因在于：数组的大小是可变的，当旧数组的大小不够用时，Rust 会重新分配一块更大的内存空间，然后把旧数组拷贝过来。
    // 这种情况下，之前的引用显然会指向一块无效的内存

    // println!("The first element is {}", first);

    println!("The vec is {:?}", v);
}

pub fn trait_diff_vec() {
    trait IpAddr {
        fn display(&self);
    }

    struct V4(String);
    impl IpAddr for V4 {
        fn display(&self) {
            println!("ipv4: {:?}", self.0)
        }
    }
    struct V6(String);
    impl IpAddr for V6 {
        fn display(&self) {
            println!("ipv6: {:?}", self.0)
        }
    }
    let v: Vec<Box<dyn IpAddr>> = vec![
        Box::new(V4("127.0.0.1".to_string())),
        Box::new(V6("::1".to_string())),
    ];
    for i in v {
        i.display();
    }
}