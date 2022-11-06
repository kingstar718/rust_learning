mod string_demo;

fn main() {
    str_demo();
}

fn str_demo() {
    println!("==> 字符串相关 ==>");
    string_demo::slice_demo_1();
    string_demo::string_slice_1();
    string_demo::str_and_string();
    string_demo::string_index();
    string_demo::string_slice_2();
    println!();
    println!("==> 字符串相关 ==>");
}
