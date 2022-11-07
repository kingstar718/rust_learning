use crate::generics_demo::generics_demo::add;
use crate::generics_demo::{display_array, display_array2, get_point_x, largest, Summary};
use crate::generics_demo::trait_demo::{Post, Weibo};

#[test]
pub fn largest_test() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}

#[test]
pub fn add_test() {
    let a = 5;
    let b = 6;
    let c = add(a, b);
    println!("add {}", c);
}

#[test]
pub fn get_point_x_test() {
    get_point_x();
}

#[test]
pub fn display_array_test() {
    let arr: [i32; 3] = [1, 2, 3];
    display_array(&arr);
    display_array2(arr);

    let arr: [i32; 2] = [1, 2];
    display_array(&arr);
    display_array2(arr);
}

#[test]
pub fn trait_test() {
    let post = Post { title: "Rust语言简介".to_string(), author: "Sunface".to_string(), content: "Rust棒极了!".to_string() };
    let weibo = Weibo { username: "sunface".to_string(), content: "好像微博没Tweet好用".to_string() };

    println!("{}", post.summarize());
    println!("{}", weibo.summarize());
}