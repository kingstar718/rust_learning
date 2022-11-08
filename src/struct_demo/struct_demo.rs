struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

pub fn create_user() {
    // 初始化实例时，每个字段都需要进行初始化
    // 初始化时的字段顺序不需要和结构体定义时的顺序一致
    let _user = User {
        active: true,
        username: String::from("jack"),
        email: String::from("jack@qq.com"),
        sign_in_count: 1,
    };

    // 访问及修改
    // 必须要将结构体实例声明为可变的，才能修改其中的字段，Rust不支持将某个结构体某个字段标记为可变
    let mut user1 = User {
        active: true,
        username: String::from("jack"),
        email: String::from("jack@qq.com"),
        sign_in_count: 1,
    };
    user1.email = String::from("another@qq.com");

    let _user2 = build_user(String::from("jack@qq.com"), String::from("jack"));
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}


#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
}

pub fn create_file() {
    let f1 = File {
        name: String::from("f1.txt"),
        data: Vec::new(),
    };
    let f1_name = &f1.name;
    let f1_length = &f1.data.len();
    println!("{:?}", f1);
    println!("{} is {} bytes long", f1_name, f1_length);
}


// 元组结构体
pub fn tuple_struct() {
    struct Color(i32, i32, i32);
    #[derive(Debug)]
    struct Point(i32, i32, i32);
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}


// 单元结构体
pub fn unit_like_struct() {
    struct AlwaysEqual;

    let subject = AlwaysEqual;

    // 我们不关心 AlwaysEqual 的字段数据，只关心它的行为，因此将它声明为单元结构体，然后再为它实现某个特征
    // impl SomeTrait for AlwaysEqual {
    //
    // }
}


// 如果需要结构体从其他对象借用数据，就需要引入生命周期（lifetimes）的新概念
// 简而言之，生命周期能确保结构体的作用范围要比它所借用的数据的作用范围要小
struct User2<'a> {
    username: &'a str,
}


// Display
// Debug {:?} {:#?} 1、手动实现Debug  2、使用derive派生实现

// dbg!宏，拿走表达式的所有权，然后打印出相应的文件名、行号等 debug 信息，当然还有我们需要的表达式的求值结果。
// 除此之外，它最终还会把表达式值的所有权返回！

pub fn struct_display() {

    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    let scale = 2;
    let rectangle = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    dbg!(&rectangle);
    println!("{:?}", rectangle);
    println!("{:#?}", rectangle);
}